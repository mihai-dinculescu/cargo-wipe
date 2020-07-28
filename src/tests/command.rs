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
    let err = FolderNameEnum::from_str(folder_name_string).err().unwrap();

    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "Valid options are: rust | target | node | node_modules"
    );
}
