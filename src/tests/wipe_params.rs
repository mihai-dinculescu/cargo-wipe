use parameterized::parameterized;
use std::path::PathBuf;

use crate::command::{Args, LanguageEnum};
use crate::wipe_params::WipeParams;

#[parameterized(
    args = {
        Args { wipe: false, language: LanguageEnum::Node, ignores: Vec::new() },
        Args { wipe: true, language: LanguageEnum::Node, ignores: Vec::new() },
        Args { wipe: true, language: LanguageEnum::Node, ignores: vec![PathBuf::from("example/path")] },
        Args { wipe: false, language: LanguageEnum::Rust, ignores: Vec::new() },
        Args { wipe: true, language: LanguageEnum::Rust, ignores: Vec::new() },
        Args { wipe: true, language: LanguageEnum::Rust, ignores: vec![PathBuf::from("example/path")] },
    },
)]
fn test_wipe_params(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            wipe: args.wipe,
            path: std::env::current_dir().unwrap(),
            language: args.language,
            ignores: args.ignores,
        }
    );
}
