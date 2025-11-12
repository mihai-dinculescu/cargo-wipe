use std::io::stdout;

use clap::Parser;

pub mod command;
pub mod dir_helpers;
pub mod wipe;
pub mod wipe_params;
pub mod writer;

use crate::command::Command;
use crate::wipe::Wipe;
use crate::wipe_params::WipeParams;

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let command = Command::parse();

    match command {
        Command::Wipe(args) => {
            let params = WipeParams::new(&args)?;
            Wipe::new(&mut stdout, &params).run()?;
        }
    }

    Ok(())
}
