use parameterized::parameterized;
use std::io::Cursor;
use std::path::PathBuf;
use yansi::Paint;

use crate::command::FolderNameEnum;
use crate::tests::helpers::path::TestPath;
use crate::wipe::{Wipe, WipeParams, SPACING_FILES, SPACING_SIZE};

#[parameterized(
    folder_name = {
        FolderNameEnum::NodeModules, FolderNameEnum::NodeModules,
        FolderNameEnum::Target, FolderNameEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn run_with_hits(folder_name: FolderNameEnum, wipe: bool) {
    let test_path = TestPath::new(3, &folder_name);

    let params = WipeParams {
        wipe,
        path: PathBuf::from(&test_path),
        folder_name: folder_name.clone(),
        ignores: Vec::new(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff, &params).run().unwrap();

    let output = std::str::from_utf8(&buff.get_ref()).unwrap();
    println!("{}", output);

    // header
    let expected = format!("{}", Paint::green("[DRY RUN]").bold());
    assert_eq!(output.contains(&expected), !wipe);

    let expected = format!("{}", Paint::red("[WIPING]").bold());
    assert_eq!(output.contains(&expected), wipe);

    let expected = format!(r#""{}""#, Paint::cyan(folder_name));
    assert!(output.contains(&expected));

    // body
    // hits should be listed and wiped if wipe is true
    for path in &test_path.hits {
        let expected = String::from(path.to_str().unwrap());
        assert_eq!(output.contains(&expected), true);
        assert_eq!(path.exists(), !wipe);
    }

    // misses should not be listed and not wiped
    for path in &test_path.misses {
        let expected = String::from(path.to_str().unwrap());
        assert_eq!(output.contains(&expected), false);
        assert_eq!(path.exists(), true);
    }

    // summary should be displayed
    let expected = format!("{:>files$}", Paint::cyan("Files #"), files = SPACING_FILES);
    let output = output.replacen(&expected, "", 1);
    assert!(output.contains(&expected));

    let expected = format!("{:>size$}", Paint::cyan("Size (MB)"), size = SPACING_SIZE);
    let output = output.replacen(&expected, "", 1);

    let expected = format!("{:>size$}", Paint::cyan("Size"), size = SPACING_SIZE);
    assert!(output.contains(&expected));

    let expected = format!("{}", Paint::cyan(test_path.path.display()));
    let output = &output.replacen(&expected, "", 1);
    assert!(output.contains(&expected));

    // footer
    if wipe {
        let expected = format!("{}", Paint::green("All clear!"));
        assert!(output.contains(&expected));
    } else {
        let expected = format!(
            "Run {} to wipe all folders found. {}",
            Paint::red(format!("cargo wipe {} -w", params.folder_name)),
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
fn run_no_hits(folder_name: FolderNameEnum, wipe: bool) {
    let test_path = TestPath::new(0, &folder_name);

    let params = WipeParams {
        wipe,
        path: PathBuf::from(&test_path),
        folder_name: folder_name,
        ignores: Vec::new(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff, &params).run().unwrap();

    let output = std::str::from_utf8(&buff.get_ref()).unwrap();
    println!("{}", output);

    // body
    let expected = format!("{}", Paint::cyan("Files #"));
    assert!(!output.contains(&expected));

    let expected = format!("{}", Paint::cyan("Size"));
    assert!(!output.contains(&expected));

    let expected = format!("{}", Paint::cyan(test_path.path.display()));
    let output = &output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    // summary should not be displayed
    let expected = format!("{:>files$}", Paint::cyan("Files #"), files = SPACING_FILES);
    let output = output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    let expected = format!("{:>size$}", Paint::cyan("Size (MB)"), size = SPACING_SIZE);
    let output = output.replacen(&expected, "", 1);

    let expected = format!("{:>size$}", Paint::cyan("Size"), size = SPACING_SIZE);
    assert!(!output.contains(&expected));

    let expected = format!("{}", Paint::cyan(test_path.path.display()));
    let output = &output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    // footer
    let expected = format!("{}", Paint::green("Nothing found!"));
    assert!(output.contains(&expected));
}
