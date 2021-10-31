#[cfg(target_os = "linux")]
mod wipe_permissions_tests {
    use parameterized::parameterized;
    use std::io::Cursor;
    use std::path::PathBuf;

    use crate::command::FolderNameEnum;
    use crate::tests::helpers::test_run::TestRun;
    use crate::wipe::{Wipe, WipeParams};

    #[parameterized(
        folder_name = {
            FolderNameEnum::Target, FolderNameEnum::Target,
        },
        wipe = { false, true },
    )]
    fn rust_with_inaccessible_folders(folder_name: FolderNameEnum, wipe: bool) {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let test_run = TestRun::new(&folder_name, 3, 0);

        let params = WipeParams {
            wipe,
            path: PathBuf::from(&test_run),
            folder_name,
            ignores: Vec::new(),
        };

        let first_hit = test_run.hits.first().unwrap().clone();

        let permissions = fs::Permissions::from_mode(0o000);
        fs::set_permissions(&first_hit, permissions).unwrap();

        let mut buff = Cursor::new(Vec::new());
        Wipe::new(&mut buff, &params).run().unwrap();

        let output = std::str::from_utf8(buff.get_ref()).unwrap();
        println!("{}", output);

        // hits should be listed and wiped if wipe is true
        for path in &test_run.hits {
            let expected = String::from(path.to_str().unwrap());

            if path.to_str() == first_hit.to_str() {
                assert!(!output.contains(&expected));
                assert!(path.exists());
            } else {
                assert!(output.contains(&expected));
                assert_eq!(path.exists(), !wipe);
            }
        }

        // revert chmod
        let permissions = fs::Permissions::from_mode(0o777);
        fs::set_permissions(&first_hit, permissions).unwrap();
    }

    #[parameterized(
        folder_name = {
            FolderNameEnum::NodeModules, FolderNameEnum::NodeModules,
        },
        wipe = { false, true },
    )]
    fn node_with_inaccessible_folders(folder_name: FolderNameEnum, wipe: bool) {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let test_run = TestRun::new(&folder_name, 3, 0);

        let params = WipeParams {
            wipe,
            path: PathBuf::from(&test_run),
            folder_name,
            ignores: Vec::new(),
        };

        let first_hit = test_run.hits.first().unwrap().clone();

        let permissions = fs::Permissions::from_mode(0o000);
        fs::set_permissions(&first_hit, permissions).unwrap();

        let mut buff = Cursor::new(Vec::new());
        Wipe::new(&mut buff, &params).run().unwrap();

        let output = std::str::from_utf8(buff.get_ref()).unwrap();
        println!("{}", output);

        // hits should be listed and wiped if wipe is true
        for path in &test_run.hits {
            let expected = String::from(path.to_str().unwrap());

            assert!(output.contains(&expected));

            if path.to_str() == first_hit.to_str() {
                assert!(path.exists());
            } else {
                assert_eq!(path.exists(), !wipe);
            }
        }

        // revert chmod
        let permissions = fs::Permissions::from_mode(0o777);
        fs::set_permissions(&first_hit, permissions).unwrap();
    }
}
