use rand::distributions::Alphanumeric;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::path::{Path, PathBuf};

use crate::opts::FolderNameEnum;

#[derive(Debug)]
pub struct TestPath {
    rng: ThreadRng,
    pub path: PathBuf,
    pub hits: Vec<PathBuf>,
    pub misses: Vec<PathBuf>,
}

impl Drop for TestPath {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

impl From<&TestPath> for PathBuf {
    fn from(test_path: &TestPath) -> Self {
        test_path.path.clone()
    }
}

impl TestPath {
    pub fn new(hits_count: u32, folder_name: &FolderNameEnum) -> Self {
        let mut test_path = TestPath::generate_parent();

        test_path.generate_hits(hits_count, folder_name);
        test_path.generate_no_hits();
        test_path.generate_opposite(folder_name);
        test_path.generate_invalid(folder_name);
        test_path.generate_partial(folder_name);

        test_path
    }

    pub fn generate_hits(&mut self, hits_count: u32, folder_name: &FolderNameEnum) {
        for _ in 0..hits_count {
            let name: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();

            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&folder_name.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            if folder_name == &FolderNameEnum::Target {
                let file_path = path.join(".rustc_info.json");
                std::fs::File::create(file_path).unwrap();
            }

            self.hits.push(path);
        }
    }

    pub fn generate_no_hits(&mut self) {
        for _ in 0..5 {
            let name: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();

            let path = self.path.join(Path::new(&name));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    pub fn generate_opposite(&mut self, folder_name: &FolderNameEnum) {
        let opposite = if matches!(folder_name, FolderNameEnum::NodeModules) {
            "target"
        } else {
            "node_modules"
        };

        let name: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();

        let path = self.path.join(Path::new(&name)).join(Path::new(opposite));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    pub fn generate_invalid(&mut self, folder_name: &FolderNameEnum) {
        if folder_name == &FolderNameEnum::Target {
            let name: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();

            let path = self
                .path
                .join(Path::new(&name))
                .join(Path::new(&folder_name.to_string()));

            std::fs::create_dir_all(&path).unwrap();

            self.misses.push(path);
        }
    }

    pub fn generate_partial(&mut self, folder_name: &FolderNameEnum) {
        let name: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();
        let name_inner: String = self.rng.sample_iter(&Alphanumeric).take(16).collect();
        let name_inner = format!("{}_{}", folder_name, name_inner);

        let path = self
            .path
            .join(Path::new(&name))
            .join(Path::new(&name_inner));

        std::fs::create_dir_all(&path).unwrap();

        self.misses.push(path);
    }

    pub fn generate_parent() -> Self {
        let rng = thread_rng();
        let name: String = rng.sample_iter(&Alphanumeric).take(16).collect();

        let path = std::env::temp_dir()
            .join(Path::new(".cargo-wipe-tests"))
            .join(Path::new(&name));

        std::fs::create_dir_all(&path).unwrap();

        TestPath {
            rng,
            path,
            hits: Vec::new(),
            misses: Vec::new(),
        }
    }
}
