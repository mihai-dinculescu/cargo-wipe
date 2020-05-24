use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::opts::FolderNameEnum;

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

    pub fn size_formatted(&self) -> String {
        let num = self.size / 1024_usize.pow(2);
        num.to_formatted_string(&Locale::en)
    }
}

fn is_valid_target(path: PathBuf, folder_name: &FolderNameEnum) -> bool {
    if folder_name == &FolderNameEnum::Target {
        let file_path = path.join(".rustc_info.json");
        return file_path.exists();
    }

    true
}

pub type PathsResult = io::Result<Vec<Result<String, io::Error>>>;

pub fn get_paths_to_delete(path: impl Into<PathBuf>, folder_name: &FolderNameEnum) -> PathsResult {
    fn walk(dir: io::Result<fs::ReadDir>, folder_name: &FolderNameEnum) -> PathsResult {
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

                let size = match file.metadata()? {
                    data if data.is_dir() => {
                        if file.file_name() == folder_name.to_string()[..] {
                            if is_valid_target(file.path(), &folder_name) {
                                acc.push(Ok(file.path().display().to_string()));
                            }
                            acc
                        } else {
                            acc.append(&mut walk(fs::read_dir(file.path()), folder_name)?);
                            acc
                        }
                    }
                    _ => acc,
                };

                Ok(size)
            },
        )
    }

    walk(fs::read_dir(path.into()), folder_name)
}

pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<DirInfo> {
    fn walk(mut dir: fs::ReadDir) -> io::Result<DirInfo> {
        dir.try_fold(DirInfo::new(0, 0, 0), |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => walk(fs::read_dir(file.path())?)?,
                data => DirInfo::new(1, 1, data.len() as usize),
            };

            Ok(DirInfo::new(
                acc.dir_count + 1,
                acc.file_count + size.file_count,
                acc.size + size.size,
            ))
        })
    }

    walk(fs::read_dir(path.into())?)
}
