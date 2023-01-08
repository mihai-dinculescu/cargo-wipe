use parameterized::parameterized;
use std::path::PathBuf;

use crate::command::{Args, LanguageEnum};
use crate::wipe::WipeParams;

#[parameterized(
    args = {
        Args { wipe: false, language_input: LanguageEnum::NodeModules, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::NodeModules, ignores: Vec::new() },
        Args { wipe: false, language_input: LanguageEnum::Node, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::Node, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::Node, ignores: vec![PathBuf::from("example/path")] },
    },
)]
fn node(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            wipe: args.wipe,
            path: std::env::current_dir().unwrap(),
            language: args.language_input,
            ignores: args.ignores,
        }
    );
}

#[parameterized(
    args = {
        Args { wipe: false, language_input: LanguageEnum::Target, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::Target, ignores: Vec::new() },
        Args { wipe: false, language_input: LanguageEnum::Rust, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::Rust, ignores: Vec::new() },
        Args { wipe: true, language_input: LanguageEnum::Rust, ignores: vec![PathBuf::from("example/path")] },
    },
)]
fn rust(args: Args) {
    let params = WipeParams::new(&args).unwrap();

    assert_eq!(
        params,
        WipeParams {
            wipe: args.wipe,
            path: std::env::current_dir().unwrap(),
            language: args.language_input,
            ignores: args.ignores,
        }
    );
}
