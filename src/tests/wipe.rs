use parameterized::parameterized;
use std::path::PathBuf;
use std::{io::Cursor, println};
use yansi::Paint;

use crate::command::LanguageEnum;
use crate::tests::helpers::test_run::TestRun;
use crate::wipe::{SPACING_FILES, SPACING_SIZE, Wipe, WipeParams};

#[parameterized(
    language = {
        LanguageEnum::NodeModules, LanguageEnum::NodeModules,
        LanguageEnum::Target, LanguageEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn run_with_hits(language: LanguageEnum, wipe: bool) {
    let test_run = TestRun::new(&language, 3, 0);

    let params = WipeParams {
        wipe,
        path: PathBuf::from(&test_run),
        language: language.clone(),
        ignores: Vec::new(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff, &params).run().unwrap();

    let output = std::str::from_utf8(buff.get_ref()).unwrap();
    println!("{output}");

    // header
    let expected = format!("{}", Paint::green("[DRY RUN]").bold());
    assert_eq!(output.contains(&expected), !wipe);

    let expected = format!("{}", Paint::red("[WIPING]").bold());
    assert_eq!(output.contains(&expected), wipe);

    let expected = format!(r#""{}""#, Paint::cyan(language));
    assert!(output.contains(&expected));

    // body
    // hits should be listed and wiped if wipe is true
    for path in &test_run.hits {
        let expected = String::from(path.to_str().unwrap());
        assert!(output.contains(&expected));
        assert_eq!(path.exists(), !wipe);
    }

    // misses should not be listed and not wiped
    for path in &test_run.misses {
        let expected = String::from(path.to_str().unwrap());
        assert!(!output.contains(&expected));
        assert!(path.exists());
    }

    // summary should be displayed
    let expected = format!("{:>files$}", Paint::cyan("Files #"), files = SPACING_FILES);
    let output = output.replacen(&expected, "", 1);
    assert!(output.contains(&expected));

    let expected = format!("{:>size$}", Paint::cyan("Size (MB)"), size = SPACING_SIZE);
    let output = output.replacen(&expected, "", 1);

    let expected = format!("{:>size$}", Paint::cyan("Size"), size = SPACING_SIZE);
    assert!(output.contains(&expected));

    let expected = format!("{}", Paint::cyan(test_run.path.display()));
    let output = &output.replacen(&expected, "", 1);
    assert!(output.contains(&expected));

    let expected = format!("{}", Paint::yellow("Ignored"));
    assert!(!output.contains(&expected));

    // footer
    if wipe {
        let expected = format!("{}", Paint::green("All clear!"));
        assert!(output.contains(&expected));
    } else {
        let expected = format!(
            "Run {} to wipe all folders found. {}",
            Paint::red(format!("cargo wipe {} -w", params.language)),
            Paint::red("USE WITH CAUTION!")
        );
        assert!(output.contains(&expected));
    }
}

#[parameterized(
    language = {
        LanguageEnum::NodeModules, LanguageEnum::NodeModules,
        LanguageEnum::Target, LanguageEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn run_no_hits(language: LanguageEnum, wipe: bool) {
    let test_run = TestRun::new(&language, 0, 0);

    let params = WipeParams {
        wipe,
        path: PathBuf::from(&test_run),
        language,
        ignores: Vec::new(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff, &params).run().unwrap();

    let output = std::str::from_utf8(buff.get_ref()).unwrap();
    println!("{output}");

    // body
    let expected = format!("{}", Paint::cyan("Files #"));
    assert!(!output.contains(&expected));

    let expected = format!("{}", Paint::cyan("Size"));
    assert!(!output.contains(&expected));

    let expected = format!("{}", Paint::cyan(test_run.path.display()));
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

    let expected = format!("{}", Paint::cyan(test_run.path.display()));
    let output = &output.replacen(&expected, "", 1);
    assert!(!output.contains(&expected));

    // footer
    let expected = format!("{}", Paint::green("Nothing found!"));
    assert!(output.contains(&expected));
}

#[parameterized(
    language = {
        LanguageEnum::NodeModules, LanguageEnum::NodeModules,
        LanguageEnum::Target, LanguageEnum::Target,
    },
    wipe = { false, true, false, true },
)]
fn run_with_ignores(language: LanguageEnum, wipe: bool) {
    let test_run = TestRun::new(&language, 3, 3);

    let params = WipeParams {
        wipe,
        path: PathBuf::from(&test_run),
        language,
        ignores: test_run.ignores.clone(),
    };

    let mut buff = Cursor::new(Vec::new());
    Wipe::new(&mut buff, &params).run().unwrap();

    let output = std::str::from_utf8(buff.get_ref()).unwrap();
    let lines = output.lines();
    println!("{output}");

    // body
    // hits should be listed and wiped if wipe is true
    for path in &test_run.hits {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_some());
        let line = line.unwrap();

        assert!(line.contains(&expected));
        assert!(!line.contains("[Ignored]"));
        assert_eq!(path.exists(), !wipe);
    }

    // ignores should be listed and not wiped if wipe is true
    for path in &test_run.ignores {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_some());
        let line = line.unwrap();

        assert!(line.contains(&expected));
        assert!(line.contains("[Ignored]"));
        assert!(path.exists());
    }

    // misses should not be listed and not wiped
    for path in &test_run.misses {
        let expected = String::from(path.to_str().unwrap());
        let mut lines = lines.clone();
        let line = lines.find(|l| l.contains(&expected));

        assert!(line.is_none());
        assert!(path.exists());
    }

    // summary should be displayed
    let expected = format!("{}", Paint::yellow("Ignored"));
    assert!(output.contains(&expected));
}
