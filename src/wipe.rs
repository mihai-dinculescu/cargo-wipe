use anyhow::{anyhow, Error};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::{env, fmt::Display};
use yansi::Paint;

use crate::command::Args;
use crate::configuration::config::Config;
use crate::configuration::file_util::load_config;
use crate::configuration::file_util::save_config;
use crate::configuration::language_option::LanguageOption;
use crate::dir_helpers::{dir_size, get_paths_to_delete, DirInfo};

pub const SPACING_FILES: usize = 12;
pub const SPACING_SIZE: usize = 18;
pub const SPACING_PATH: usize = 9;

#[derive(Debug, PartialEq, Eq)]
pub struct WipeParams {
    pub wipe: bool,
    pub path: PathBuf,
    pub language_input: String,
    pub ignores: Vec<PathBuf>,
}

impl WipeParams {
    pub fn new(args: &Args) -> io::Result<Self> {
        let path = env::current_dir()?;

        Ok(Self {
            wipe: args.wipe,
            path,
            language_input: args.language_input.clone(),
            ignores: args.ignores.clone(),
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
    config: Config,
    previous_info: Option<DirInfo>,
    wipe_info: Option<DirInfo>,
    ignore_info: Option<DirInfo>,
}

impl<'a, W> Wipe<'a, W>
where
    W: io::Write,
{
    pub fn new(stdout: &'a mut W, params: &'a WipeParams) -> Self {
        let config = match load_config() {
            Ok(config) => config,
            Err(e) => {
                write!(
                    stdout,
                    "{} {} {}\n",
                    Paint::yellow("[WARNING]").bold(),
                    "Error loading config! Will generate default config. Error: ",
                    e
                )
                .expect("Couldn't write to stdout!");
                let new_config = Config::default();
                let save_result = save_config(&new_config);

                if let Err(e) = save_result {
                    write!(
                        stdout,
                        "{} {} {}\n",
                        Paint::red("[ERROR]").bold(),
                        "Error saving generated config! Error: ",
                        e
                    )
                    .expect("Couldn't write to stdout!");
                }

                new_config
            }
        };

        Self {
            stdout,
            params,
            config,
            previous_info: None,
            wipe_info: None,
            ignore_info: None,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let language_option = self.write_assert_language_option()?.clone();

        self.write_header(&language_option)?;
        self.write_content(&language_option)?;
        self.write_footer()?;

        Ok(())
    }

    fn write_header(&mut self, language_option: &LanguageOption) -> Result<(), anyhow::Error> {
        if self.params.wipe {
            write!(self.stdout, "{}", Paint::red("[WIPING]").bold())?;
        } else {
            write!(self.stdout, "{}", Paint::green("[DRY RUN]").bold())?;
        }

        writeln!(
            self.stdout,
            r#" Recursively searching for all "{}" folders in {}..."#,
            Paint::cyan(language_option),
            Paint::cyan(self.params.path.display()),
        )?;

        self.stdout.flush()?;

        Ok(())
    }

    fn write_content(&mut self, language_option: &LanguageOption) -> io::Result<()> {
        let paths_to_delete = get_paths_to_delete(&self.params.path, &language_option)?;
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

            self.previous_info = Some(dir_size(&self.params.path)?);
        }

        let mut wipe_info = DirInfo::new(paths_to_delete.len(), 0, 0);
        let mut ignore_info = DirInfo::new(0, 0, 0);
        let paths_ignored = self
            .params
            .ignores
            .iter()
            .map(|p| p.display().to_string().to_lowercase())
            .collect::<Vec<_>>();

        for path in paths_to_delete {
            let dir_info = dir_size(&path);

            let ignored = paths_ignored
                .iter()
                .any(|p| path.to_lowercase().starts_with(p));

            if let Ok(dir_info) = dir_info {
                self.write_spaced_line(
                    dir_info.file_count_formatted(),
                    dir_info.size_formatted_mb(),
                    "",
                    &path,
                )?;

                if ignored {
                    ignore_info.dir_count += 1;
                    ignore_info.file_count += dir_info.file_count;
                    ignore_info.size += dir_info.size;
                } else {
                    wipe_info.file_count += dir_info.file_count;
                    wipe_info.size += dir_info.size;
                }
            } else {
                self.write_spaced_line("?", "?", "", &path)?;
            }

            if ignored {
                write!(self.stdout, " {}", Paint::yellow("[Ignored]"))?;
            } else if self.params.wipe {
                let r = fs::remove_dir_all(path);

                if let Err(e) = r {
                    write!(self.stdout, " {}", Paint::red(&format!("[{}]", e)))?;
                }
            }

            writeln!(self.stdout)?;

            self.stdout.flush()?;
        }

        self.wipe_info = Some(wipe_info);
        self.ignore_info = Some(ignore_info);

        Ok(())
    }

    fn write_summary(&mut self) -> io::Result<()> {
        let previous_info = self.previous_info.expect("this should never be None");
        let wipe_info = self.wipe_info.expect("this should never be None");
        let ignore_info = self.ignore_info.expect("this should never be None");

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

        if ignore_info.dir_count > 0 {
            self.writeln_spaced_line(
                Paint::yellow(ignore_info.file_count_formatted()),
                Paint::yellow(ignore_info.size_formatted_flex()),
                "",
                Paint::yellow("Ignored"),
            )?;
        }

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

    fn write_footer(&mut self) -> io::Result<()> {
        let wipe_info = self.wipe_info.as_ref().expect("this should never be None");

        writeln!(self.stdout)?;

        if wipe_info.dir_count > 0 {
            self.write_summary()?;

            if !self.params.wipe {
                writeln!(
                    self.stdout,
                    "Run {} to wipe all folders found. {}",
                    Paint::red(format!("cargo wipe {} -w", self.params.language_input)),
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

    fn write_assert_language_option(&mut self) -> Result<&LanguageOption, Error> {
        let language_option: Option<&LanguageOption> =
            self.config.get_option(&self.params.language_input);

        if language_option.is_none() {
            writeln!(
                self.stdout,
                r#" Could not find language option "{}" in config. Please add it to the config file."#,
                Paint::red(&self.params.language_input),
            )?;
            return Err(anyhow!("Could not find language option in config"));
        }

        Ok(language_option.unwrap())
    }
}
