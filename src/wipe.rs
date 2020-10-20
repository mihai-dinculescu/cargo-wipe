use std::fs;
use std::io;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::{env, fmt::Display};
use yansi::Paint;

use crate::command::{Args, FolderNameEnum};
use crate::dir_helpers::{dir_size, get_paths_to_delete, DirInfo};

pub const SPACING_FILES: usize = 12;
pub const SPACING_SIZE: usize = 18;
pub const SPACING_PATH: usize = 9;

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
    previous_info: MaybeUninit<DirInfo>,
    wipe_info: MaybeUninit<DirInfo>,
}

impl<'a, W> Wipe<'a, W>
where
    W: io::Write,
{
    pub fn new(stdout: &'a mut W, params: &'a WipeParams) -> Self {
        Self {
            stdout,
            params,
            previous_info: MaybeUninit::uninit(),
            wipe_info: MaybeUninit::uninit(),
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
            r#" Recursively searching for all "{}" folders in {}..."#,
            Paint::cyan(&self.params.folder_name),
            Paint::cyan(self.params.path.display()),
        )?;

        self.stdout.flush()?;

        Ok(())
    }

    fn write_content(&mut self) -> io::Result<()> {
        let paths_to_delete = get_paths_to_delete(&self.params.path, &self.params.folder_name)?;
        let paths_to_delete = paths_to_delete
            .iter()
            .filter_map(|p| match p {
                Ok(item) => Some(item),
                _ => None,
            })
            .collect::<Vec<_>>();

        if !paths_to_delete.is_empty() {
            writeln!(self.stdout)?;

            self.writeln_spaced_line(
                Paint::cyan("Files #"),
                Paint::cyan("Size (MB)"),
                "",
                Paint::cyan("Path"),
            )?;

            self.previous_info = MaybeUninit::new(dir_size(&self.params.path)?);
        }

        let dir_count = &paths_to_delete.len();
        let mut file_count = 0_usize;
        let mut size = 0_usize;

        for path in paths_to_delete {
            let dir_info = dir_size(&path);

            if let Ok(dir_info) = dir_info {
                self.write_spaced_line(
                    dir_info.file_count_formatted(),
                    dir_info.size_formatted_mb(),
                    "",
                    &path,
                )?;

                file_count += dir_info.file_count;
                size += dir_info.size;
            } else {
                self.write_spaced_line("?", "?", "", &path)?;
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

        self.wipe_info = MaybeUninit::new(DirInfo::new(*dir_count, file_count, size));

        Ok(())
    }

    fn write_footer(&mut self) -> io::Result<()> {
        let wipe_info = unsafe { self.wipe_info.assume_init() };

        writeln!(self.stdout)?;

        if wipe_info.dir_count > 0 {
            self.write_summary()?;

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

    fn write_summary(&mut self) -> io::Result<()> {
        let (wipe_info, previous_info) = unsafe {
            (
                self.wipe_info.assume_init(),
                self.previous_info.assume_init(),
            )
        };

        let after = DirInfo {
            dir_count: previous_info.dir_count - wipe_info.dir_count,
            file_count: previous_info.file_count - wipe_info.file_count,
            size: previous_info.size - wipe_info.size,
        };

        self.writeln_spaced_line(
            Paint::cyan("Files #"),
            Paint::cyan("Size"),
            "",
            Paint::cyan(self.params.path.display()),
        )?;

        let label = if self.params.wipe {
            "Previously"
        } else {
            "Currently"
        };

        self.writeln_spaced_line(
            Paint::default(previous_info.file_count_formatted()),
            Paint::default(previous_info.size_formatted_flex()),
            "",
            Paint::default(label),
        )?;

        let label = if self.params.wipe {
            "Wiped"
        } else {
            "Can wipe"
        };

        self.writeln_spaced_line(
            Paint::red(wipe_info.file_count_formatted()),
            Paint::red(wipe_info.size_formatted_flex()),
            "",
            Paint::red(label),
        )?;

        let label = if self.params.wipe {
            "Now"
        } else {
            "After wipe"
        };

        self.writeln_spaced_line(
            Paint::green(after.file_count_formatted()),
            Paint::green(after.size_formatted_flex()),
            "",
            Paint::green(label),
        )?;

        writeln!(self.stdout)?;

        self.stdout.flush()?;

        Ok(())
    }

    fn write_spaced_line(
        &mut self,
        column_1: impl Display,
        column_2: impl Display,
        column_3: impl Display,
        column_4: impl Display,
    ) -> io::Result<()> {
        write!(
            self.stdout,
            r#"{:>files$}{:>size$}{:>path$}{}"#,
            column_1,
            column_2,
            column_3,
            column_4,
            files = SPACING_FILES,
            size = SPACING_SIZE,
            path = SPACING_PATH,
        )?;

        Ok(())
    }

    fn writeln_spaced_line(
        &mut self,
        column_1: impl Display,
        column_2: impl Display,
        column_3: impl Display,
        column_4: impl Display,
    ) -> io::Result<()> {
        writeln!(
            self.stdout,
            r#"{:>files$}{:>size$}{:>path$}{}"#,
            column_1,
            column_2,
            column_3,
            column_4,
            files = SPACING_FILES,
            size = SPACING_SIZE,
            path = SPACING_PATH,
        )?;

        Ok(())
    }
}
