use structopt::StructOpt;

use wipe::opts::Command;
use wipe::wipe::wipe_folders;

fn main() -> anyhow::Result<()> {
    let command = Command::from_args();

    match command {
        Command::Wipe(args) => wipe_folders(&args)?,
    }

    Ok(())
}
