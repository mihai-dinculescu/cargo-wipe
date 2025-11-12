use std::{fmt::Display, io};

use yansi::Paint as _;

use crate::command::DirectoryEnum;
use crate::dir_helpers::DirInfo;
use crate::wipe_params::WipeParams;

pub const SPACING_FILES: usize = 12;
pub const SPACING_SIZE: usize = 18;
pub const SPACING_PATH: usize = 9;

#[derive(Debug)]
pub struct Writer<'a, W>
where
    W: io::Write,
{
    stdout: &'a mut W,
}

impl<'a, W> Writer<'a, W>
where
    W: io::Write,
{
    pub fn new(stdout: &'a mut W) -> Self {
        Self { stdout }
    }

    pub fn write_header(&mut self, params: &WipeParams) -> io::Result<()> {
        let directory: DirectoryEnum = params.language.clone().into();

        let title = if params.wipe {
            "[WIPING]".red()
        } else {
            "[DRY RUN]".green()
        };
        write!(self.stdout, "{}", title.bold())?;

        writeln!(
            self.stdout,
            r#" Recursively searching for all "{}" folders in {}..."#,
            &directory.cyan(),
            params.path.display().cyan(),
        )?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_content_header(&mut self) -> io::Result<()> {
        writeln!(self.stdout)?;
        self.writeln_spaced_line("Files #".cyan(), "Size (MB)".cyan(), "", "Path".cyan())?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_content_line(
        &mut self,
        path: &str,
        dir_info: Result<DirInfo, io::Error>,
        ignored: bool,
        result: Option<io::Error>,
    ) -> io::Result<()> {
        if let Ok(dir_info) = dir_info {
            self.write_spaced_line(
                dir_info.file_count_formatted(),
                dir_info.size_formatted_mb(),
                "",
                path,
            )?;
        } else {
            self.write_spaced_line("?", "?", "", path)?;
        }

        if ignored {
            write!(self.stdout, " {}", "[Ignored]".yellow())?;
        }

        if let Some(e) = result {
            write!(self.stdout, " {}", format!("[{e}]").red())?;
        }

        writeln!(self.stdout)?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn write_summary(
        &mut self,
        params: &WipeParams,
        wipe_info: &DirInfo,
        ignore_info: &DirInfo,
        previous_info: &Option<DirInfo>,
    ) -> io::Result<()> {
        writeln!(self.stdout)?;

        if wipe_info.dir_count > 0 {
            let previous_info = previous_info.expect("this should never be None");

            let after = DirInfo {
                dir_count: previous_info.dir_count - wipe_info.dir_count,
                file_count: previous_info.file_count - wipe_info.file_count,
                size: previous_info.size - wipe_info.size,
            };

            self.writeln_spaced_line(
                "Files #".cyan(),
                "Size".cyan(),
                "",
                params.path.display().cyan(),
            )?;

            let label = if params.wipe {
                "Previously"
            } else {
                "Currently"
            };

            self.writeln_spaced_line(
                previous_info.file_count_formatted(),
                previous_info.size_formatted_flex(),
                "",
                label,
            )?;

            if ignore_info.dir_count > 0 {
                self.writeln_spaced_line(
                    ignore_info.file_count_formatted().yellow(),
                    ignore_info.size_formatted_flex().yellow(),
                    "",
                    "Ignored".yellow(),
                )?;
            }

            let label = if params.wipe { "Wiped" } else { "Can wipe" };

            self.writeln_spaced_line(
                wipe_info.file_count_formatted().red(),
                wipe_info.size_formatted_flex().red(),
                "",
                label.red(),
            )?;

            let label = if params.wipe { "Now" } else { "After wipe" };

            self.writeln_spaced_line(
                after.file_count_formatted().green(),
                after.size_formatted_flex().green(),
                "",
                label.green(),
            )?;

            writeln!(self.stdout)?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    pub fn write_footer(&mut self, params: &WipeParams, wipe_info: &DirInfo) -> io::Result<()> {
        if wipe_info.dir_count > 0 {
            if params.wipe {
                writeln!(self.stdout, "{}", "All clear!".green())?
            } else {
                writeln!(
                    self.stdout,
                    "Run {} to wipe all folders found. {}",
                    format!("cargo wipe {} -w", params.language).red(),
                    "USE WITH CAUTION!".red()
                )?;
            }
        } else {
            writeln!(self.stdout, "{}", "Nothing found!".green())?
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
            r#"{column_1:>SPACING_FILES$}{column_2:>SPACING_SIZE$}{column_3:>SPACING_PATH$}{column_4}"#,
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
            r#"{column_1:>SPACING_FILES$}{column_2:>SPACING_SIZE$}{column_3:>SPACING_PATH$}{column_4}"#,
        )?;

        Ok(())
    }
}
