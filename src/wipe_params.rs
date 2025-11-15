use std::path::PathBuf;
use std::{env, io};

use crate::command::{Args, LanguageEnum};

#[derive(Debug, PartialEq, Eq)]
pub struct WipeParams {
    pub wipe: bool,
    pub path: PathBuf,
    pub language: LanguageEnum,
    pub ignores: Vec<PathBuf>,
}

impl WipeParams {
    pub fn new(args: &Args) -> io::Result<Self> {
        let path = env::current_dir()?;

        Ok(Self {
            wipe: args.wipe,
            path,
            language: args.language.clone(),
            ignores: args.ignores.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::command::{Args, LanguageEnum};
    use crate::wipe_params::WipeParams;

    #[rstest]
    #[case(Args { language: LanguageEnum::Node, wipe: false, ignores: Vec::new() })]
    #[case(Args { language: LanguageEnum::Node, wipe: true, ignores: Vec::new() })]
    #[case(Args { language: LanguageEnum::Node, wipe: true, ignores: vec![PathBuf::from("example/path")] })]
    #[case(Args { language: LanguageEnum::Rust, wipe: false, ignores: Vec::new() })]
    #[case(Args { language: LanguageEnum::Rust, wipe: true, ignores: Vec::new() })]
    #[case(Args { language: LanguageEnum::Rust, wipe: true, ignores: vec![PathBuf::from("example/path")] })]
    fn test_wipe_params(#[case] args: Args) {
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
}
