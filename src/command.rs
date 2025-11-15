use std::path::PathBuf;
use std::{fmt, io, str};

use clap::{Parser, ValueEnum, arg, command};

#[derive(Debug, Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all "target" (Rust) or
    /// "node_modules" (Node) folders that are found in the current path.
    /// Add the `-w` flag to wipe all folders found. USE WITH CAUTION!
    Wipe(Args),
}

#[derive(Debug, Parser)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    bin_name = "cargo",
    help_template = "{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}",
)]
pub struct Args {
    /// Language to target
    pub language: LanguageEnum,
    /// Caution! If set it will wipe all folders found! Unset by default
    #[arg(short, long)]
    pub wipe: bool,
    /// Absolute paths to ignore
    #[arg(short, long, value_parser)]
    pub ignores: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq, Clone, ValueEnum)]
pub enum LanguageEnum {
    Node,
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
            "node" => Ok(LanguageEnum::Node),
            "rust" => Ok(LanguageEnum::Rust),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Valid options are: node | rust",
            )),
        }
    }
}

impl fmt::Display for LanguageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageEnum::Node => write!(f, "node"),
            LanguageEnum::Rust => write!(f, "rust"),
        }
    }
}

impl From<LanguageEnum> for DirectoryEnum {
    fn from(language: LanguageEnum) -> Self {
        match language {
            LanguageEnum::Node => DirectoryEnum::NodeModules,
            LanguageEnum::Rust => DirectoryEnum::Target,
        }
    }
}

impl From<&LanguageEnum> for DirectoryEnum {
    fn from(language: &LanguageEnum) -> Self {
        match language {
            LanguageEnum::Node => DirectoryEnum::NodeModules,
            LanguageEnum::Rust => DirectoryEnum::Target,
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
