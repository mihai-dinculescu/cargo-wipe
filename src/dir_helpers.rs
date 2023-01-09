use anyhow::Error;
use num_format::{Locale, ToFormattedString};
use number_prefix::NumberPrefix;
use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

use crate::command::DirectoryEnum;
use crate::configuration::language_option::LanguageOption;

#[derive(Debug, Copy, Clone)]
pub struct DirInfo {
    pub dir_count: usize,
    pub file_count: usize,
    pub size: usize,
}

impl DirInfo {
    pub fn new(dir_count: usize, file_count: usize, size: usize) -> Self {
        DirInfo {
            dir_count,
            file_count,
            size,
        }
    }

    pub fn file_count_formatted(&self) -> String {
        self.file_count.to_formatted_string(&Locale::en)
    }

    pub fn size_formatted_mb(&self) -> String {
        let num = self.size / 1024_usize.pow(2);
        num.to_formatted_string(&Locale::en)
    }

    pub fn size_formatted_flex(&self) -> String {
        let np = NumberPrefix::binary(self.size as f64);

        match np {
            NumberPrefix::Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
            NumberPrefix::Standalone(bytes) => format!("{} bytes", bytes),
        }
    }
}

fn is_valid_target(target_path: PathBuf, language_option: &LanguageOption) -> Result<bool, Error> {
    // Check if the target folder is a match first
    let target_is_match = target_path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|file_name| language_option.target_folder_regexp.is_match(file_name))
        .unwrap_or(false);

    if !target_is_match {
        return Ok(false);
    }

    // For any additional criteria on the language option, check if everything matches.
    fn check_contents_with_matches<'a, T>(
        target_path: &PathBuf,
        matches_to_check: T,
    ) -> Result<bool, Error>
    where
        T: IntoIterator<Item = &'a Regex>,
    {
        let target_contents: Vec<_> = fs::read_dir(&target_path)?
            .filter_map(|dir_entry| {
                dir_entry
                    .ok()
                    .and_then(|_dir_entry| _dir_entry.file_name().to_str().map(|s| s.to_string()))
            })
            .collect();

        Ok(matches_to_check.into_iter().all(|regexp| {
            // All listed regexp must match at least one file in the target folder
            target_contents
                .iter()
                .any(|file_name| regexp.is_match(file_name))
        }))
    }

    // If there are target folder contents to match, check to make sure our matched target is actually a folder.
    // If the target is not a folder, but there are target folder contents to match, then we should return false.
    let target_contents_match =
        if language_option.target_folder_contains_regexp.len() > 0 && target_path.is_dir() {
            check_contents_with_matches(
                &target_path,
                language_option.target_folder_contains_regexp.iter(),
            )?
        } else {
            // This nested if feels a bit ugly. Maybe there's a better flow organization here.
            if language_option.target_folder_contains_regexp.len() > 0 && target_path.is_file() {
                false
            } else {
                true
            }
        };

    let parent_path = target_path.parent();
    let parent_contents_match =
        if language_option.parent_folder_contains_regexp.len() > 0 && parent_path.is_some() {
            let parent_path = parent_path.unwrap().to_path_buf();
            check_contents_with_matches(
                &parent_path,
                language_option.parent_folder_contains_regexp.iter(),
            )?
        } else {
            // This nested if feels a bit ugly. Maybe there's a better flow organization here.
            if language_option.parent_folder_contains_regexp.len() > 0 && target_path.is_file() {
                false
            } else {
                true
            }
        };

    Ok(target_is_match && target_contents_match && parent_contents_match)
}

pub type PathsResult = io::Result<Vec<Result<String, io::Error>>>;

pub fn get_paths_to_delete(
    path: impl Into<PathBuf>,
    language_option: &LanguageOption,
) -> PathsResult {
    fn walk(dir: io::Result<fs::ReadDir>, language_option: &LanguageOption) -> PathsResult {
        let mut dir = match dir {
            Ok(dir) => dir,
            Err(e) => {
                return Ok(vec![Err(e)]);
            }
        };

        dir.try_fold(
            Vec::new(),
            |mut acc: Vec<Result<String, io::Error>>, file| {
                let file = file?;

                let size = match file.metadata() {
                    Ok(data) if data.is_dir() => {
                        //TODO: Do we need to tell the user if the validating the target errors?
                        if is_valid_target(file.path(), language_option).unwrap_or(false) {
                            acc.push(Ok(file.path().display().to_string()));
                        } else {
                            acc.append(&mut walk(fs::read_dir(file.path()), language_option)?);
                        }
                        acc
                    }
                    _ => acc,
                };

                Ok(size)
            },
        )
    }

    walk(fs::read_dir(path.into()), language_option)
}

pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<DirInfo> {
    fn walk(dir: io::Result<fs::ReadDir>) -> io::Result<DirInfo> {
        let mut dir = match dir {
            Ok(dir) => dir,
            Err(_) => {
                return Ok(DirInfo::new(0, 0, 0));
            }
        };

        dir.try_fold(DirInfo::new(0, 0, 0), |acc, file| {
            let file = file?;

            let size = match file.metadata() {
                Ok(data) if data.is_dir() => walk(fs::read_dir(file.path()))?,
                Ok(data) => DirInfo::new(1, 1, data.len() as usize),
                _ => DirInfo::new(0, 0, 0),
            };

            Ok(DirInfo::new(
                acc.dir_count + 1,
                acc.file_count + size.file_count,
                acc.size + size.size,
            ))
        })
    }

    walk(fs::read_dir(path.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        size = { 0, 512, 1024, 1024_usize.pow(2), 1024_usize.pow(3), 1024_usize.pow(4) },
        output = { "0 bytes", "512 bytes", "1.00 KiB", "1.00 MiB", "1.00 GiB", "1.00 TiB" },
    )]
    fn size_formatted_flex(size: usize, output: &str) {
        let di = DirInfo {
            dir_count: 0,
            file_count: 0,
            size,
        };

        assert_eq!(di.size_formatted_flex(), output);
    }
}
