use std::{io::Write, path::PathBuf};
use yansi::Paint;

use crate::dir_helpers::{dir_size, get_folders, DirInfo};
use crate::opts::{Args, FolderNameEnum};

#[derive(Debug, PartialEq)]
pub struct WipeParams {
    pub wipe: bool,
    pub path: PathBuf,
    pub folder_name: String,
}

pub fn get_params(args: &Args) -> std::io::Result<WipeParams> {
    let path = std::env::current_dir()?;

    let (folder_name, wipe) = match &args.folder_name {
        FolderNameEnum::Node(opts) => ("node_modules", opts.wipe),
        FolderNameEnum::NodeModules(opts) => ("node_modules", opts.wipe),
        FolderNameEnum::Rust(opts) => ("target", opts.wipe),
        FolderNameEnum::Target(opts) => ("target", opts.wipe),
    };

    Ok(WipeParams {
        folder_name: folder_name.to_owned(),
        path,
        wipe,
    })
}

pub fn wipe_folders<W: Write>(mut stdout: &mut W, params: &WipeParams) -> std::io::Result<()> {
    write_header(&mut stdout, &params)?;
    let total = write_content(&mut stdout, &params)?;
    write_footer(&mut stdout, &params, &total)?;

    Ok(())
}

fn write_header<W: Write>(stdout: &mut W, params: &WipeParams) -> std::io::Result<()> {
    if params.wipe {
        write!(stdout, "{}", Paint::red("[WIPING]").bold())?;
    } else {
        write!(stdout, "{}", Paint::green("[DRY RUN]").bold())?;
    }

    writeln!(
        stdout,
        r#" Recursively searching for all "{}" folders in {} ..."#,
        Paint::yellow(&params.folder_name),
        Paint::yellow(params.path.display()),
    )?;

    writeln!(stdout)?;

    writeln!(
        stdout,
        r#"{:>18}{:>18}{:>9}{}"#,
        Paint::default("Files #").bold(),
        Paint::default("Size (MB)").bold(),
        "",
        Paint::default("Path").bold()
    )?;

    stdout.flush()?;

    Ok(())
}

fn write_content<W: Write>(stdout: &mut W, params: &WipeParams) -> std::io::Result<DirInfo> {
    let folders_to_delete = get_folders(&params.path, &params.folder_name)?;

    let dir_count = &folders_to_delete.len();
    let mut file_count = 0_usize;
    let mut size = 0_usize;

    for folder in folders_to_delete {
        let dir_info = dir_size(&folder)?;

        writeln!(
            stdout,
            r#"{:>18}{:>18}{:>9}{}"#,
            dir_info.file_count_formatted(),
            dir_info.size_formatted(),
            "",
            &folder
        )?;

        if params.wipe {
            std::fs::remove_dir_all(folder)?;
        }

        stdout.flush()?;

        file_count += dir_info.file_count;
        size += dir_info.size;
    }

    Ok(DirInfo {
        dir_count: *dir_count,
        file_count,
        size,
    })
}

fn write_footer<W: Write>(
    stdout: &mut W,
    params: &WipeParams,
    total: &DirInfo,
) -> std::io::Result<()> {
    writeln!(stdout)?;
    writeln!(
        stdout,
        r#"{:>18}{:>18}"#,
        Paint::default("Total Files #").bold(),
        Paint::default("Total Size (MB)").bold()
    )?;
    writeln!(
        stdout,
        r#"{:>18}{:>18}"#,
        Paint::default(total.file_count_formatted()),
        Paint::default(total.size_formatted())
    )?;

    stdout.flush()?;

    writeln!(stdout)?;
    if total.dir_count > 0 {
        if !params.wipe {
            writeln!(
                stdout,
                "Run {} to wipe all folders found. {}",
                Paint::red(format!("cargo wipe {} -w", params.folder_name)),
                Paint::red("USE WITH CAUTION!")
            )?;
            if params.folder_name == "target" {
                writeln!(stdout,
                    "{} In its current form, this will remove {}, irrespective of if they are Rust folders or not!",
                    Paint::red("Warning!"),
                    Paint::red(r#"all folders named "target""#).underline()
                )?;
            }
        } else {
            writeln!(stdout, "{}", Paint::green("All clear!"))?
        }
    } else {
        writeln!(stdout, "{}", Paint::green("Nothing found!"))?
    }

    stdout.flush()?;

    Ok(())
}
