use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all <target> or <node_modules> folders that are found in the current path. Add `-w` to wipe all folders found. USE WITH CAUTION!
    Wipe(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: SubcommandEnum,
}

#[derive(Debug, StructOpt)]
pub enum SubcommandEnum {
    /// Recursively finds and optionally wipes all <node_modules> folders that are found in the current path
    #[structopt(name = "node_modules")]
    NodeModules(Opts),
    /// Alias to node_modules
    Node(Opts),
    /// Recursively finds and optionally wipes all <target> folders that are found in the current path
    Target(Opts),
    /// Alias to target
    Rust(Opts),
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// CAUTION! If set will wipe all found folders! Unset by default
    #[structopt(short, long)]
    pub wipe: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FolderNameEnum {
    NodeModules,
    Target,
}

impl ToString for FolderNameEnum {
    fn to_string(&self) -> String {
        match self {
            FolderNameEnum::NodeModules => String::from("node_modules"),
            FolderNameEnum::Target => String::from("target"),
        }
    }
}

pub struct ParsedSubcommand {
    pub folder_name: FolderNameEnum,
    pub wipe: bool,
}

impl From<&SubcommandEnum> for ParsedSubcommand {
    fn from(subcommand: &SubcommandEnum) -> Self {
        match subcommand {
            SubcommandEnum::Node(opts) => ParsedSubcommand {
                folder_name: FolderNameEnum::NodeModules,
                wipe: opts.wipe,
            },
            SubcommandEnum::NodeModules(opts) => ParsedSubcommand {
                folder_name: FolderNameEnum::NodeModules,
                wipe: opts.wipe,
            },
            SubcommandEnum::Rust(opts) => ParsedSubcommand {
                folder_name: FolderNameEnum::Target,
                wipe: opts.wipe,
            },
            SubcommandEnum::Target(opts) => ParsedSubcommand {
                folder_name: FolderNameEnum::Target,
                wipe: opts.wipe,
            },
        }
    }
}
