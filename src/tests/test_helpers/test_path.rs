use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TestPath {
    pub path: PathBuf,
    pub hits: Vec<PathBuf>,
    pub misses: Vec<PathBuf>,
}

impl TestPath {
    pub fn new(count: u32, folder_name: &str) -> Self {
        let rng = thread_rng();
        let mut test_path = TestPath::generate_parent();

        for _ in 1..count {
            let name: String = rng.sample_iter(&Alphanumeric).take(16).collect();

            let path = test_path
                .path
                .join(Path::new(&name))
                .join(Path::new(folder_name));

            std::fs::create_dir_all(&path).unwrap();

            test_path.hits.push(path);
        }

        for _ in 1..count {
            let name: String = rng.sample_iter(&Alphanumeric).take(16).collect();

            let path = test_path.path.join(Path::new(&name));

            std::fs::create_dir_all(&path).unwrap();

            test_path.misses.push(path);
        }

        test_path
    }

    pub fn generate_parent() -> Self {
        let rng = thread_rng();
        let name: String = rng.sample_iter(&Alphanumeric).take(16).collect();

        let path = std::env::current_dir()
            .unwrap()
            .join(Path::new(".mocks"))
            .join(Path::new(&name));

        std::fs::create_dir_all(&path).unwrap();

        TestPath {
            path,
            hits: Vec::new(),
            misses: Vec::new(),
        }
    }
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
