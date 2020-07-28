use std::{fmt, io, str};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all <target> or <node_modules> folders that are found in the current path. Add the `-w` flag to wipe all folders found. USE WITH CAUTION!
    Wipe(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt()]
    /// rust | target | node | node_modules
    pub folder_name: FolderNameEnum,
    /// Caution! If set it will wipe all folders found! Unset by default
    #[structopt(short, long)]
    pub wipe: bool,
}

#[derive(Debug, PartialEq, Clone, StructOpt)]
pub enum FolderNameEnum {
    #[structopt(name = "node_modules")]
    NodeModules,
    Node,
    Target,
    Rust,
}

impl str::FromStr for FolderNameEnum {
    type Err = io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "node_modules" => Ok(FolderNameEnum::NodeModules),
            "node" => Ok(FolderNameEnum::Node),
            "target" => Ok(FolderNameEnum::Target),
            "rust" => Ok(FolderNameEnum::Rust),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Valid options are: rust | target | node | node_modules",
            )),
        }
    }
}

impl fmt::Display for FolderNameEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FolderNameEnum::NodeModules => write!(f, "node_modules"),
            FolderNameEnum::Target => write!(f, "target"),
            // variations like `Node` and `Rust` should never get displayed
            _ => Err(std::fmt::Error),
        }
    }
}
