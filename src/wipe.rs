use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use yansi::Paint;

use crate::command::{Args, FolderNameEnum};
use crate::dir_helpers::{dir_size, get_paths_to_delete, DirInfo};

#[derive(Debug, PartialEq)]
pub struct WipeParams {
    pub wipe: bool,
    pub path: PathBuf,
    pub folder_name: FolderNameEnum,
}

impl WipeParams {
    pub fn new(args: &Args) -> io::Result<Self> {
        let path = env::current_dir()?;

        Ok(Self {
            folder_name: match args.folder_name {
                FolderNameEnum::Node | FolderNameEnum::NodeModules => FolderNameEnum::NodeModules,
                FolderNameEnum::Rust | FolderNameEnum::Target => FolderNameEnum::Target,
            },
            path,
            wipe: args.wipe,
        })
    }
}

#[derive(Debug)]
pub struct Wipe<'a, W>
where
    W: io::Write,
{
    stdout: &'a mut W,
    params: &'a WipeParams,
    total: Option<DirInfo>,
}

impl<'a, W> Wipe<'a, W>
where
    W: io::Write,
{
    pub fn new(stdout: &'a mut W, params: &'a WipeParams) -> Self {
        Self {
            stdout,
            params,
            total: None,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.write_header()?;
        self.write_content()?;
        self.write_footer()?;

        Ok(())
    }

    fn write_header(&mut self) -> io::Result<()> {
        if self.params.wipe {
            write!(self.stdout, "{}", Paint::red("[WIPING]").bold())?;
        } else {
            write!(self.stdout, "{}", Paint::green("[DRY RUN]").bold())?;
        }

        writeln!(
            self.stdout,
            r#" Recursively searching for all "{}" folders in {} ..."#,
            Paint::yellow(&self.params.folder_name),
            Paint::yellow(self.params.path.display()),
        )?;

        writeln!(self.stdout)?;

        writeln!(
            self.stdout,
            r#"{:>18}{:>18}{:>9}{}"#,
            Paint::default("Files #").bold(),
            Paint::default("Size (MB)").bold(),
            "",
            Paint::default("Path").bold()
        )?;

        self.stdout.flush()?;

        Ok(())
    }

    fn write_content(&mut self) -> io::Result<()> {
        let paths_to_delete = get_paths_to_delete(&self.params.path, &self.params.folder_name)?;

        let dir_count = &paths_to_delete.len();
        let mut file_count = 0_usize;
        let mut size = 0_usize;

        for path in paths_to_delete {
            if let Ok(path) = path {
                let dir_info = dir_size(&path);

                if let Ok(dir_info) = dir_info {
                    write!(
                        self.stdout,
                        r#"{:>18}{:>18}{:>9}{}"#,
                        dir_info.file_count_formatted(),
                        dir_info.size_formatted_mb(),
                        "",
                        &path
                    )?;

                    file_count += dir_info.file_count;
                    size += dir_info.size;
                } else {
                    write!(self.stdout, r#"{:>18}{:>18}{:>9}{}"#, "?", "?", "", &path)?;
                }

                if self.params.wipe {
                    let r = fs::remove_dir_all(path);

                    if let Err(e) = r {
                        write!(self.stdout, " {}", Paint::red(e))?;
                    }
                }

                writeln!(self.stdout)?;

                self.stdout.flush()?;
            }
        }

        self.total = Some(DirInfo {
            dir_count: *dir_count,
            file_count,
            size,
        });

        Ok(())
    }

    fn write_footer(&mut self) -> io::Result<()> {
        let total = self.total.as_ref().expect("this should never be None");

        writeln!(self.stdout)?;
        writeln!(
            self.stdout,
            r#"{:>18}{:>18}"#,
            Paint::default("Total Files #").bold(),
            Paint::default("Total Size").bold()
        )?;
        writeln!(
            self.stdout,
            r#"{:>18}{:>18}"#,
            Paint::default(total.file_count_formatted()),
            Paint::default(total.size_formatted_flex())
        )?;

        self.stdout.flush()?;

        writeln!(self.stdout)?;
        if total.dir_count > 0 {
            if !self.params.wipe {
                writeln!(
                    self.stdout,
                    "Run {} to wipe all folders found. {}",
                    Paint::red(format!("cargo wipe {} -w", self.params.folder_name)),
                    Paint::red("USE WITH CAUTION!")
                )?;
            } else {
                writeln!(self.stdout, "{}", Paint::green("All clear!"))?
            }
        } else {
            writeln!(self.stdout, "{}", Paint::green("Nothing found!"))?
        }

        self.stdout.flush()?;

        Ok(())
    }
}
