#[cfg(target_os = "linux")]
mod wipe_permissions_tests {
    use std::io::Cursor;
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::command::LanguageEnum;
    use crate::tests::helpers::test_run::TestRun;
    use crate::wipe::Wipe;
    use crate::wipe_params::WipeParams;

    #[rstest]
    #[case(LanguageEnum::Node, false)]
    #[case(LanguageEnum::Node, true)]
    #[case(LanguageEnum::Rust, false)]
    #[case(LanguageEnum::Rust, true)]
    fn test_with_readonly_folders(#[case] language: LanguageEnum, #[case] wipe: bool) {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let test_run = TestRun::new(&language, 3, 0);

        let params = WipeParams {
            wipe,
            path: PathBuf::from(&test_run),
            language,
            ignores: Vec::new(),
        };

        let first_hit = test_run.hits.first().unwrap().clone();
        let first_hit_parent = first_hit.parent().unwrap();

        let permissions = fs::Permissions::from_mode(0o555);
        fs::set_permissions(first_hit_parent, permissions).unwrap();

        let mut buff = Cursor::new(Vec::new());
        Wipe::new(&mut buff, &params).run().unwrap();

        let output = std::str::from_utf8(buff.get_ref()).unwrap();
        println!("{output}");

        // hits should be listed
        // and wiped if wipe is true and delete permissions are present
        for (i, path) in test_run.hits.iter().enumerate() {
            let expected = String::from(path.to_str().unwrap());

            assert!(output.contains(&expected));
            assert_eq!(path.exists(), !wipe || i == 0);
        }

        // revert the permissions change for cleanup
        let permissions = fs::Permissions::from_mode(0o777);
        fs::set_permissions(first_hit_parent, permissions).unwrap();
    }
}
