use parameterized::parameterized;
use std::{io::Cursor, path::PathBuf};
use yansi::Paint;

use crate::tests::test_helpers::test_path::TestPath;
use crate::wipe::{wipe_folders, WipeParams};

#[parameterized(folder_name = { "node_modules", "node_modules", "target", "target" }, wipe = { false, true, false, true })]
fn node(folder_name: &str, wipe: bool) {
    let test_path = TestPath::new(3, folder_name);

    let params = WipeParams {
        folder_name: folder_name.to_owned(),
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

    let expected = format!(r#""{}""#, Paint::yellow(folder_name));
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
}
