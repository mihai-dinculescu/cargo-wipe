use parameterized::parameterized;
use std::{io::Cursor, path::PathBuf};
use yansi::Paint;

use crate::opts::FolderNameEnum;
use crate::tests::test_helpers::test_path::TestPath;
use crate::wipe::{wipe_folders, WipeParams};

#[parameterized(
    folder_name = {
        FolderNameEnum::NodeModules, FolderNameEnum::NodeModules,
        FolderNameEnum::Target, FolderNameEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn wipe_with_hits(folder_name: FolderNameEnum, wipe: bool) {
    let test_path = TestPath::new(3, &folder_name);

    let params = WipeParams {
        folder_name: folder_name.clone(),
        path: PathBuf::from(&test_path),
        wipe,
    };
    let mut buff = Cursor::new(Vec::new());

    wipe_folders(&mut buff, &params).unwrap();

    let output = std::str::from_utf8(&buff.get_ref()).unwrap();

    let expected = format!("{}", Paint::green("[DRY RUN]").bold());
    assert_eq!(output.contains(&expected), !wipe);

    let expected = format!("{}", Paint::red("[WIPING]").bold());
    assert_eq!(output.contains(&expected), wipe);

    let expected = format!(r#""{}""#, Paint::yellow(folder_name.to_string()));
    assert!(output.contains(&expected));

    // hits should be listed and wiped if wipe is true
    for path in &test_path.hits {
        let expected = String::from(path.to_str().unwrap());
        assert!(output.contains(&expected));
        assert_eq!(path.exists(), !wipe);
    }

    // misses should not be listed and not wiped
    for path in &test_path.misses {
        let expected = String::from(path.to_str().unwrap());
        assert!(!output.contains(&expected));
        assert!(path.exists());
    }

    if wipe {
        let expected = format!("{}", Paint::green("All clear!"));
        assert!(output.contains(&expected));
    } else {
        let expected = format!(
            "Run {} to wipe all folders found. {}",
            Paint::red(format!("cargo wipe {} -w", params.folder_name.to_string())),
            Paint::red("USE WITH CAUTION!")
        );
        assert!(output.contains(&expected));
    }
}

#[parameterized(
    folder_name = {
        FolderNameEnum::NodeModules, FolderNameEnum::NodeModules,
        FolderNameEnum::Target, FolderNameEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn wipe_no_hits(folder_name: FolderNameEnum, wipe: bool) {
    let test_path = TestPath::new(0, &folder_name);

    let params = WipeParams {
        folder_name,
        path: PathBuf::from(&test_path),
        wipe,
    };
    let mut buff = Cursor::new(Vec::new());

    wipe_folders(&mut buff, &params).unwrap();

    let output = std::str::from_utf8(&buff.get_ref()).unwrap();

    let expected = format!("{}", Paint::green("Nothing found!"));
    assert!(output.contains(&expected));
}
