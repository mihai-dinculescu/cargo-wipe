#[cfg(target_os = "linux")]
use parameterized::parameterized;
#[cfg(target_os = "linux")]
use std::io::Cursor;
#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use crate::command::FolderNameEnum;
#[cfg(target_os = "linux")]
use crate::tests::test_helpers::test_path::TestPath;
#[cfg(target_os = "linux")]
use crate::wipe::{wipe_folders, WipeParams};

#[cfg(target_os = "linux")]
#[parameterized(
    folder_name = {
        FolderNameEnum::Target, FolderNameEnum::Target,
    },
    wipe = { false, true },
)]
fn rust_with_inaccessible_folders(folder_name: FolderNameEnum, wipe: bool) {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let test_path = TestPath::new(3, &folder_name);

    let params = WipeParams {
        folder_name,
        path: PathBuf::from(&test_path),
        wipe,
    };

    let first_hit = test_path.hits.first().unwrap().clone();

    let permissions = fs::Permissions::from_mode(0o000);
    fs::set_permissions(&first_hit, permissions).unwrap();

    let mut buff = Cursor::new(Vec::new());
    wipe_folders(&mut buff, &params).unwrap();
    let output = std::str::from_utf8(&buff.get_ref()).unwrap();
    println!("{}", output);

    // hits should be listed and wiped if wipe is true
    for path in &test_path.hits {
        let expected = String::from(path.to_str().unwrap());

        if path.to_str() == first_hit.to_str() {
            assert_eq!(output.contains(&expected), false);
            assert_eq!(path.exists(), true);
        } else {
            assert_eq!(output.contains(&expected), true);
            assert_eq!(path.exists(), !wipe);
        }
    }

    // revert chmod
    let permissions = fs::Permissions::from_mode(0o777);
    fs::set_permissions(&first_hit, permissions).unwrap();
}

#[cfg(target_os = "linux")]
#[parameterized(
    folder_name = {
        FolderNameEnum::NodeModules, FolderNameEnum::NodeModules,
    },
    wipe = { false, true },
)]
fn node_with_inaccessible_folders(folder_name: FolderNameEnum, wipe: bool) {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let test_path = TestPath::new(3, &folder_name);

    let params = WipeParams {
        folder_name,
        path: PathBuf::from(&test_path),
        wipe,
    };

    let first_hit = test_path.hits.first().unwrap().clone();

    let permissions = fs::Permissions::from_mode(0o000);
    fs::set_permissions(&first_hit, permissions).unwrap();

    let mut buff = Cursor::new(Vec::new());
    wipe_folders(&mut buff, &params).unwrap();
    let output = std::str::from_utf8(&buff.get_ref()).unwrap();
    println!("{}", output);

    // hits should be listed and wiped if wipe is true
    for path in &test_path.hits {
        let expected = String::from(path.to_str().unwrap());

        if path.to_str() == first_hit.to_str() {
            assert_eq!(output.contains(&expected), true);
            assert_eq!(path.exists(), true);
        } else {
            assert_eq!(output.contains(&expected), true);
            assert_eq!(path.exists(), !wipe);
        }
    }

    // revert chmod
    let permissions = fs::Permissions::from_mode(0o777);
    fs::set_permissions(&first_hit, permissions).unwrap();
}
