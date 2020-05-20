use parameterized::parameterized;

use crate::opts::{Args, FolderNameEnum, Opts};
use crate::wipe::{get_params, WipeParams};

#[parameterized(folder_name = {
    FolderNameEnum::Node(Opts { wipe: false }), FolderNameEnum::NodeModules(Opts { wipe: false }),
    FolderNameEnum::Node(Opts { wipe: true }), FolderNameEnum::NodeModules(Opts { wipe: true })
}, wipe = { false, false, true, true })]
fn node(folder_name: FolderNameEnum, wipe: bool) {
    let args = Args { folder_name };

    let params = get_params(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: "node_modules".to_owned(),
            path: std::env::current_dir().unwrap(),
            wipe,
        }
    );
}

#[parameterized(folder_name = {
    FolderNameEnum::Rust(Opts { wipe: false }), FolderNameEnum::Target(Opts { wipe: false }),
    FolderNameEnum::Rust(Opts { wipe: true }), FolderNameEnum::Target(Opts { wipe: true })
}, wipe = { false, false, true, true })]
fn rust(folder_name: FolderNameEnum, wipe: bool) {
    let args = Args { folder_name };

    let params = get_params(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            folder_name: "target".to_owned(),
            path: std::env::current_dir().unwrap(),
            wipe,
        }
    );
}
