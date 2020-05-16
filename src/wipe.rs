use crate::opts::{Args, FolderNameEnum};
use num_format::{Locale, ToFormattedString};
use std::path::PathBuf;
use yansi::Paint;

pub struct DirInfo {
    pub file_count: u64,
    pub size: u64,
}

impl DirInfo {
    pub fn new(file_count: u64, size: u64) -> Self {
        DirInfo { file_count, size }
    }

    pub fn file_count_formatted(&self) -> String {
        let num: String = self.file_count.to_formatted_string(&Locale::en);
        num
    }

    pub fn size_formatted(&self) -> String {
        let num = self.size / 1024_u64.pow(2);
        let num = num.to_formatted_string(&Locale::en);
        num
    }
}

pub fn wipe_folders(args: &Args) -> std::io::Result<()> {
    let path = std::env::current_dir()?;

    let (folder_target, wipe) = match &args.folder_name {
        FolderNameEnum::Node(opts) => ("node_modules", opts.wipe),
        FolderNameEnum::Rust(opts) => ("target", opts.wipe),
    };

    if wipe {
        print!("{}", Paint::red("[WIPING]").bold());
    } else {
        print!("{}", Paint::green("[DRY RUN]").bold());
    }

    println!(
        r#" Recursively searching for all "{}" folders in {} ..."#,
        Paint::yellow(folder_target),
        Paint::yellow(path.display()),
    );

    let folders_to_delete = get_folders(path, folder_target)?;

    println!(
        r#"{:>18}{:>18}{:>9}{}"#,
        Paint::default("Files #").bold(),
        Paint::default("Size (MB)").bold(),
        "",
        Paint::default("Path").bold()
    );

    let mut file_count = 0_u64;
    let mut size = 0_u64;

    for folder in folders_to_delete {
        let dir_info = dir_size(&folder)?;

        println!(
            r#"{:>18}{:>18}{:>9}{}"#,
            dir_info.file_count_formatted(),
            dir_info.size_formatted(),
            "",
            &folder
        );

        if wipe {
            std::fs::remove_dir_all(folder).unwrap();
        }

        file_count += dir_info.file_count;
        size += dir_info.size;
    }

    let total = DirInfo { file_count, size };

    println!("");
    println!(
        r#"{:>18}{:>18}"#,
        Paint::default("Total Files #").bold(),
        Paint::default("Total Size (MB)").bold()
    );
    println!(
        r#"{:>18}{:>18}"#,
        Paint::default(total.file_count_formatted()),
        Paint::default(total.size_formatted())
    );

    if total.file_count > 0 {
        if !wipe {
            println!(
                "Run {} to wipe all folders found. {}",
                Paint::red(format!("cargo wipe node -w")),
                Paint::red("USE WITH CAUTION!")
            )
        } else {
            println!("{}", Paint::green("All clear!"))
        }
    } else {
        println!("{}", Paint::green("Nothing found!"))
    }

    Ok(())
}

fn get_folders(path: impl Into<PathBuf>, folder_name: &str) -> std::io::Result<Vec<String>> {
    fn walk(mut dir: std::fs::ReadDir, folder_name: &str) -> std::io::Result<Vec<String>> {
        dir.try_fold(Vec::new(), |mut acc: Vec<String>, file| {
            let file = file?;

            let size = match file.metadata()? {
                data if data.is_dir() => {
                    if file.file_name() == folder_name {
                        acc.push(file.path().display().to_string());
                        acc
                    } else {
                        acc.append(&mut walk(std::fs::read_dir(file.path())?, folder_name)?);
                        acc
                    }
                }
                _ => acc,
            };

            Ok(size)
        })
    }

    walk(std::fs::read_dir(path.into())?, folder_name)
}

fn dir_size(path: impl Into<PathBuf>) -> std::io::Result<DirInfo> {
    fn walk(mut dir: std::fs::ReadDir) -> std::io::Result<DirInfo> {
        dir.try_fold(DirInfo::new(0, 0), |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => walk(std::fs::read_dir(file.path())?)?,
                data => DirInfo::new(1, data.len()),
            };

            Ok(DirInfo::new(
                acc.file_count + size.file_count,
                acc.size + size.size,
            ))
        })
    }

    walk(std::fs::read_dir(path.into())?)
}
