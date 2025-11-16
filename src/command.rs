use std::path::PathBuf;
use std::{fmt, io, str};

use clap::{Parser, ValueEnum, arg, command};

#[derive(Debug, Parser)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all "target" (Rust),
    /// "node_modules" (Node), or ".terraform" (Terraform) folders that
    /// are found in the current path.
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
    Terraform,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirectoryEnum {
    NodeModules,
    Target,
    Terraform,
}

impl str::FromStr for LanguageEnum {
    type Err = io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().trim() {
            "node" => Ok(LanguageEnum::Node),
            "rust" => Ok(LanguageEnum::Rust),
            "terraform" => Ok(LanguageEnum::Terraform),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Valid options are: node | rust | terraform",
            )),
        }
    }
}

impl fmt::Display for LanguageEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageEnum::Node => write!(f, "node"),
            LanguageEnum::Rust => write!(f, "rust"),
            LanguageEnum::Terraform => write!(f, "terraform"),
        }
    }
}

impl From<LanguageEnum> for DirectoryEnum {
    fn from(language: LanguageEnum) -> Self {
        match language {
            LanguageEnum::Node => DirectoryEnum::NodeModules,
            LanguageEnum::Rust => DirectoryEnum::Target,
            LanguageEnum::Terraform => DirectoryEnum::Terraform,
        }
    }
}

impl From<&LanguageEnum> for DirectoryEnum {
    fn from(language: &LanguageEnum) -> Self {
        match language {
            LanguageEnum::Node => DirectoryEnum::NodeModules,
            LanguageEnum::Rust => DirectoryEnum::Target,
            LanguageEnum::Terraform => DirectoryEnum::Terraform,
        }
    }
}

impl fmt::Display for DirectoryEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectoryEnum::NodeModules => write!(f, "node_modules"),
            DirectoryEnum::Target => write!(f, "target"),
            DirectoryEnum::Terraform => write!(f, ".terraform"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io, str::FromStr};

    use rstest::rstest;

    use crate::command::{DirectoryEnum, LanguageEnum};

    #[rstest]
    #[case("node", LanguageEnum::Node)]
    #[case("rust", LanguageEnum::Rust)]
    #[case("terraform", LanguageEnum::Terraform)]
    #[case("RUST", LanguageEnum::Rust)]
    #[case("ruSt ", LanguageEnum::Rust)]
    fn language_string_to_enum(#[case] language_string: &str, #[case] language_enum: LanguageEnum) {
        assert_eq!(
            LanguageEnum::from_str(language_string).unwrap(),
            language_enum
        );
    }

    #[rstest]
    #[case("node-modules")]
    #[case("rustt")]
    fn language_string_to_enum_error(#[case] language_string: &str) {
        let result = LanguageEnum::from_str(language_string);
        let err = result.err().unwrap();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(
            err.to_string(),
            "Valid options are: node | rust | terraform"
        );
    }

    #[rstest]
    #[case(LanguageEnum::Node, "node")]
    #[case(LanguageEnum::Rust, "rust")]
    #[case(LanguageEnum::Terraform, "terraform")]
    fn language_enum_to_string(#[case] language_enum: LanguageEnum, #[case] language_string: &str) {
        assert_eq!(language_enum.to_string(), language_string);
    }

    #[rstest]
    #[case(LanguageEnum::Node, DirectoryEnum::NodeModules)]
    #[case(LanguageEnum::Rust, DirectoryEnum::Target)]
    #[case(LanguageEnum::Terraform, DirectoryEnum::Terraform)]
    fn language_enum_to_directory_enum(
        #[case] language_enum: LanguageEnum,
        #[case] expected_directory_enum: DirectoryEnum,
    ) {
        let directory_enum: DirectoryEnum = language_enum.into();
        assert_eq!(directory_enum, expected_directory_enum);
    }

    #[rstest]
    #[case(DirectoryEnum::NodeModules, "node_modules")]
    #[case(DirectoryEnum::Target, "target")]
    #[case(DirectoryEnum::Terraform, ".terraform")]
    fn directory_enum_to_string(
        #[case] directory_enum: DirectoryEnum,
        #[case] directory_string: &str,
    ) {
        assert_eq!(directory_enum.to_string(), directory_string);
    }
}
