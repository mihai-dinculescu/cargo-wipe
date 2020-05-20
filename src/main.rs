use std::io::stdout;
use structopt::StructOpt;

pub mod dir_helpers;
pub mod opts;
pub mod wipe;

use crate::opts::Command;
use crate::wipe::{get_params, wipe_folders};

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let command = Command::from_args();

    match command {
        Command::Wipe(args) => {
            let params = get_params(&args)?;
            wipe_folders(&mut stdout, &params)?;
        }
    }

    Ok(())
}
