use rand::{Rng, prelude::ThreadRng, rng};
use rand_distr::Alphanumeric;
use std::path::{Path, PathBuf};

use crate::command::{DirectoryEnum, LanguageEnum};

#[derive(Debug)]
pub struct TestRun {
    rng: ThreadRng,
    pub path: PathBuf,
    pub hits: Vec<PathBuf>,
    pub ignores: Vec<PathBuf>,
    pub misses: Vec<PathBuf>,
}

impl Drop for TestRun {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

impl From<&TestRun> for PathBuf {
    fn from(test_path: &TestRun) -> Self {
        test_path.path.clone()
    }
}

impl TestRun {
    pub fn new(language: &LanguageEnum, hits_count: u32, ignores_count: u32) -> Self {
        let mut rng = rng();
        let name = TestRun::generate_folder_name(&mut rng);

        let path = std::env::temp_dir()
            .join(Path::new(".cargo-wipe-tests"))
            .join(Path::new(&name));

        std::fs::create_dir_all(&path).unwrap();

        let mut run = TestRun {
            rng,
            path,
            hits: Vec::new(),
            ignores: Vec::new(),
            misses: Vec::new(),
        };

        run.generate_hits(language, hits_count);
        run.generate_ignores(language, ignores_count);
        run.generate_no_hits();
        run.generate_opposite(language);
        run.generate_invalid(language);
        run.generate_partial(language);

        run
    }

    fn generate_hits(&mut self, language: &LanguageEnum, hits_count: u32) {
        let directory: DirectoryEnum = language.into();

        for _ in 0..hits_count {
            let name = TestRun::generate_folder_name(&mut self.rng);
            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&directory.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            if language == &LanguageEnum::Rust {
                let file_path = path.join(".rustc_info.json");
                std::fs::File::create(file_path).unwrap();
            }

            self.hits.push(path);
        }
    }

    fn generate_ignores(&mut self, language: &LanguageEnum, ignores_count: u32) {
        let directory: DirectoryEnum = language.into();

        for _ in 0..ignores_count {
            let name = TestRun::generate_folder_name(&mut self.rng);
            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&directory.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            if language == &LanguageEnum::Rust {
                let file_path = path.join(".rustc_info.json");
                std::fs::File::create(file_path).unwrap();
            }

            self.ignores.push(path);
        }
    }

    fn generate_no_hits(&mut self) {
        for _ in 0..5 {
            let name = TestRun::generate_folder_name(&mut self.rng);
            let path = self.path.join(Path::new(&name));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    fn generate_opposite(&mut self, language: &LanguageEnum) {
        let opposite = if matches!(language, LanguageEnum::Node) {
            "target"
        } else {
            "node_modules"
        };

        let name = TestRun::generate_folder_name(&mut self.rng);
        let path = self.path.join(Path::new(&name)).join(Path::new(opposite));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    fn generate_invalid(&mut self, language: &LanguageEnum) {
        let directory: DirectoryEnum = language.into();

        if language == &LanguageEnum::Rust {
            let name = TestRun::generate_folder_name(&mut self.rng);
            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&directory.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    fn generate_partial(&mut self, language: &LanguageEnum) {
        let directory: DirectoryEnum = language.into();

        let name = TestRun::generate_folder_name(&mut self.rng);
        let name_inner = TestRun::generate_folder_name(&mut self.rng);
        let name_inner = format!("{directory}_{name_inner}");

        let path = self
            .path
            .join(Path::new(&name))
            .join(Path::new(&name_inner));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    fn generate_folder_name(rng: &mut impl Rng) -> String {
        rng.sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
