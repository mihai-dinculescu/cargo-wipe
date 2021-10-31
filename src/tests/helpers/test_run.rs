use rand::distributions::Alphanumeric;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::path::{Path, PathBuf};

use crate::command::LanguageEnum;

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
        let mut test_path = TestRun::generate_parent();

        test_path.generate_hits(language, hits_count);
        test_path.generate_ignores(language, ignores_count);
        test_path.generate_no_hits();
        test_path.generate_opposite(language);
        test_path.generate_invalid(language);
        test_path.generate_partial(language);

        test_path
    }

    pub fn generate_hits(&mut self, language: &LanguageEnum, hits_count: u32) {
        for _ in 0..hits_count {
            let name: String = (&mut self.rng)
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&language.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            if language == &LanguageEnum::Target {
                let file_path = path.join(".rustc_info.json");
                std::fs::File::create(file_path).unwrap();
            }

            self.hits.push(path);
        }
    }

    pub fn generate_ignores(&mut self, language: &LanguageEnum, ignores_count: u32) {
        for _ in 0..ignores_count {
            let name: String = (&mut self.rng)
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&language.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            if language == &LanguageEnum::Target {
                let file_path = path.join(".rustc_info.json");
                std::fs::File::create(file_path).unwrap();
            }

            self.ignores.push(path);
        }
    }

    pub fn generate_no_hits(&mut self) {
        for _ in 0..5 {
            let name: String = (&mut self.rng)
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let path = self.path.join(Path::new(&name));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    pub fn generate_opposite(&mut self, language: &LanguageEnum) {
        let opposite = if matches!(language, LanguageEnum::NodeModules) {
            "target"
        } else {
            "node_modules"
        };

        let name: String = (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let path = self.path.join(Path::new(&name)).join(Path::new(opposite));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    pub fn generate_invalid(&mut self, language: &LanguageEnum) {
        if language == &LanguageEnum::Target {
            let name: String = (&mut self.rng)
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&language.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    pub fn generate_partial(&mut self, language: &LanguageEnum) {
        let name: String = (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let name_inner: String = (&mut self.rng)
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let name_inner = format!("{}_{}", language, name_inner);

        let path = self
            .path
            .join(Path::new(&name))
            .join(Path::new(&name_inner));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    pub fn generate_parent() -> Self {
        let mut rng = thread_rng();
        let name: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let path = std::env::temp_dir()
            .join(Path::new(".cargo-wipe-tests"))
            .join(Path::new(&name));

        std::fs::create_dir_all(&path).unwrap();

        TestRun {
            rng,
            path,
            hits: Vec::new(),
            ignores: Vec::new(),
            misses: Vec::new(),
        }
    }
}
