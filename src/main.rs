use std::io::stdout;

use clap::StructOpt;

pub mod command;
pub mod dir_helpers;
pub mod wipe;

use crate::{
    command::Command,
    wipe::{Wipe, WipeParams},
};

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
