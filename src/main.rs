use std::io::stdout;
use structopt::StructOpt;

pub mod command;
pub mod configuration;
pub mod dir_helpers;
pub mod wipe;

use crate::command::Command;
use crate::wipe::{Wipe, WipeParams};

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let command = Command::from_args();

    match command {
        Command::Wipe(args) => {
            let params = WipeParams::new(&args)?;
            Wipe::new(&mut stdout, &params).run()?;
        }
    }

    Ok(())
}
