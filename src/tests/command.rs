use std::{io, str::FromStr};

use parameterized::parameterized;

use crate::command::{DirectoryEnum, LanguageEnum};

#[parameterized(
    language_string = {
        "node",
        "rust",
        "RUST",
        "ruSt ",
    },
    language_enum = {
        LanguageEnum::Node,
        LanguageEnum::Rust,
        LanguageEnum::Rust,
        LanguageEnum::Rust,
    },
)]
fn language_string_to_enum(language_string: &str, language_enum: LanguageEnum) {
    assert_eq!(
        LanguageEnum::from_str(language_string).unwrap(),
        language_enum
    );
}

#[parameterized(
    language_string = {
        "node-modules",
        "rustt",
    },
)]
fn language_string_to_enum_error(language_string: &str) {
    let result = LanguageEnum::from_str(language_string);
    let err = result.err().unwrap();

    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(err.to_string(), "Valid options are: node | rust");
}

#[parameterized(
    language_enum = {
        LanguageEnum::Node,
        LanguageEnum::Rust,
    },
    language_string = {
        "node",
        "rust",
    },
)]
fn language_enum_to_string(language_enum: LanguageEnum, language_string: &str) {
    assert_eq!(language_enum.to_string(), language_string);
}

#[parameterized(
    language_enum = {
        LanguageEnum::Node,
        LanguageEnum::Rust,
    },
    expected_directory_enum = {
        DirectoryEnum::NodeModules,
        DirectoryEnum::Target,
    },
)]
fn language_enum_to_directory_enum(
    language_enum: LanguageEnum,
    expected_directory_enum: DirectoryEnum,
) {
    let directory_enum: DirectoryEnum = language_enum.into();
    assert_eq!(directory_enum, expected_directory_enum);
}

#[parameterized(
    directory_enum = {
        DirectoryEnum::NodeModules,
        DirectoryEnum::Target,
    },
    directory_string = {
        "node_modules",
        "target",
    },
)]
fn directory_enum_to_string(directory_enum: DirectoryEnum, directory_string: &str) {
    assert_eq!(directory_enum.to_string(), directory_string);
}
