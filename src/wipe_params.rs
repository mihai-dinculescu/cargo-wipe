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
