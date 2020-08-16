use std::{io, str::FromStr};

use parameterized::parameterized;

use crate::command::FolderNameEnum;

#[parameterized(
    folder_name_string = {
        "node_modules",
        "node",
        "target",
        "rust",
        "target ",
    },
    folder_name_enum = {
        FolderNameEnum::NodeModules,
        FolderNameEnum::Node,
        FolderNameEnum::Target,
        FolderNameEnum::Rust,
        FolderNameEnum::Target,
    },
)]
fn folder_name_string_to_enum(folder_name_string: &str, folder_name_enum: FolderNameEnum) {
    assert_eq!(
        FolderNameEnum::from_str(folder_name_string).unwrap(),
        folder_name_enum
    );
}

#[parameterized(
    folder_name_string = {
        "node-modules",
        "NODE",
    },
)]
fn folder_name_string_to_enum_error(folder_name_string: &str) {
    let result = FolderNameEnum::from_str(folder_name_string);
    let err = result.err().unwrap();

    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "Valid options are: rust | target | node | node_modules"
    );
}

#[parameterized(
    folder_name_enum = {
        FolderNameEnum::NodeModules,
        FolderNameEnum::Target,
    },
    folder_name_string = {
        "node_modules",
        "target",
    },
)]
fn folder_name_enum_to_string(folder_name_enum: FolderNameEnum, folder_name_string: &str) {
    assert_eq!(folder_name_enum.to_string(), folder_name_string);
}

#[parameterized(
    folder_name_enum = {
        FolderNameEnum::Node,
        FolderNameEnum::Rust,
    },
)]
#[should_panic]
fn folder_name_enum_to_string_panic(folder_name_enum: FolderNameEnum) {
    let _ = folder_name_enum.to_string();
}
