use parameterized::parameterized;

use crate::command::{Args, FolderNameEnum};
use crate::wipe::WipeParams;

#[parameterized(
    args = {
        Args { wipe: false, folder_name: FolderNameEnum::Node, ignores: Vec::new(), },
        Args { wipe: true, folder_name: FolderNameEnum::Node, ignores: Vec::new(),  },
        Args { wipe: false, folder_name: FolderNameEnum::NodeModules, ignores: Vec::new(),  },
        Args { wipe: true, folder_name: FolderNameEnum::NodeModules, ignores: Vec::new(),  },
    },
)]
fn node(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            wipe: args.wipe,
            path: std::env::current_dir().unwrap(),
            folder_name: FolderNameEnum::NodeModules,
            ignores: args.ignores,
        }
    );
}

#[parameterized(
    args = {
        Args { wipe: false, folder_name: FolderNameEnum::Rust, ignores: Vec::new(), },
        Args { wipe: true, folder_name: FolderNameEnum::Rust, ignores: Vec::new(),  },
        Args { wipe: false, folder_name: FolderNameEnum::Target, ignores: Vec::new(),  },
        Args { wipe: true, folder_name: FolderNameEnum::Target, ignores: Vec::new(),  },
    },
)]
fn rust(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            wipe: args.wipe,
            path: std::env::current_dir().unwrap(),
            folder_name: FolderNameEnum::Target,
            ignores: args.ignores,
        }
    );
}
