use parameterized::parameterized;

use crate::command::{Args, FolderNameEnum};
use crate::wipe::WipeParams;

#[parameterized(
    args = {
        Args { folder_name: FolderNameEnum::Node, wipe: false },
        Args { folder_name: FolderNameEnum::Node, wipe: true },
        Args { folder_name: FolderNameEnum::NodeModules, wipe: false },
        Args { folder_name: FolderNameEnum::NodeModules, wipe: true },
    },
)]
fn node(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: FolderNameEnum::NodeModules,
            path: std::env::current_dir().unwrap(),
            wipe: args.wipe,
        }
    );
}

#[parameterized(
    args = {
        Args { folder_name: FolderNameEnum::Rust, wipe: false },
        Args { folder_name: FolderNameEnum::Rust, wipe: true },
        Args { folder_name: FolderNameEnum::Target, wipe: false },
        Args { folder_name: FolderNameEnum::Target, wipe: true },
    },
)]
fn rust(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: FolderNameEnum::Target,
            path: std::env::current_dir().unwrap(),
            wipe: args.wipe,
        }
    );
}
