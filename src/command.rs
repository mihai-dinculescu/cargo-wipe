use std::{fmt, io, path, str};

use clap::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all <target> or <node_modules>
    /// folders that are found in the current path. Add the `-w` flag to wipe
    /// all folders found. USE WITH CAUTION!
    Wipe(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    /// rust | node
    pub language: LanguageEnum,
    /// Caution! If set it will wipe all folders found! Unset by default
    #[structopt(short, long)]
    pub wipe: bool,
    /// Absolute paths to ignore
    #[structopt(short, long, parse(from_os_str))]
    pub ignores: Vec<path::PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Clone, StructOpt)]
pub enum LanguageEnum {
    #[structopt(name = "node_modules")]
    NodeModules,
    Node,
    Target,
    Rust,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirectoryEnum {
    NodeModules,
    Target,
}

impl str::FromStr for LanguageEnum {
    type Err = io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().trim() {
            "node_modules" => Ok(LanguageEnum::NodeModules),
            "node" => Ok(LanguageEnum::Node),
            "target" => Ok(LanguageEnum::Target),
            "rust" => Ok(LanguageEnum::Rust),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Valid options are: rust | node",
            )),
        }
    }
}

impl fmt::Display for LanguageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageEnum::Node => write!(f, "node"),
            LanguageEnum::NodeModules => write!(f, "node_modules"),
            LanguageEnum::Rust => write!(f, "rust"),
            LanguageEnum::Target => write!(f, "target"),
        }
    }
}

impl From<LanguageEnum> for DirectoryEnum {
    fn from(language: LanguageEnum) -> Self {
        match language {
            LanguageEnum::Node => DirectoryEnum::NodeModules,
            LanguageEnum::NodeModules => DirectoryEnum::NodeModules,
            LanguageEnum::Rust => DirectoryEnum::Target,
            LanguageEnum::Target => DirectoryEnum::Target,
        }
    }
}

impl fmt::Display for DirectoryEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectoryEnum::NodeModules => write!(f, "node_modules"),
            DirectoryEnum::Target => write!(f, "target"),
        }
    }
}
