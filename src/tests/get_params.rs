use parameterized::parameterized;

use crate::opts::{Args, FolderNameEnum, Opts, SubcommandEnum};
use crate::wipe::{get_params, WipeParams};

#[parameterized(
    subcommand = {
        SubcommandEnum::Node(Opts { wipe: false }), SubcommandEnum::NodeModules(Opts { wipe: false }),
        SubcommandEnum::Node(Opts { wipe: true }), SubcommandEnum::NodeModules(Opts { wipe: true }),
    },
    wipe = { false, false, true, true },
)]
fn node(subcommand: SubcommandEnum, wipe: bool) {
    let args = Args { subcommand };

    let params = get_params(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: FolderNameEnum::NodeModules,
            path: std::env::current_dir().unwrap(),
            wipe,
        }
    );
}

#[parameterized(
    subcommand = {
        SubcommandEnum::Rust(Opts { wipe: false }), SubcommandEnum::Target(Opts { wipe: false }),
        SubcommandEnum::Rust(Opts { wipe: true }), SubcommandEnum::Target(Opts { wipe: true }),
    },
    wipe = { false, false, true, true },
)]
fn rust(subcommand: SubcommandEnum, wipe: bool) {
    let args = Args { subcommand };

    let params = get_params(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: FolderNameEnum::Target,
            path: std::env::current_dir().unwrap(),
            wipe,
        }
    );
}
