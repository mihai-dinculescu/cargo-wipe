use std::{collections::HashMap, path::PathBuf};

use regex::Regex;
use serde::{self, Deserialize, Serialize};
use serde_regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageOption {
    /// The name of the language option
    #[serde(skip)]
    pub option_name: String,

    /// The RegEx that matches the target folder to be removed
    #[serde(with = "serde_regex")]
    pub target_folder_regexp: Regex,

    // I wanted this and the other list to be an Option, but serde_regex doesn't like that.
    // Perhaps we can implement a custom deserializer for Option<Vec<Regex>>.
    /// The optional RegEx expressions that match what the target folder must contain
    #[serde(with = "serde_regex")]
    #[serde(default)] // If not supplied, serde supplies an empty vector.
    pub target_folder_contains_regexp: Vec<Regex>,

    /// The optional RegEx expressions that match what the target folder's parent must contain
    #[serde(with = "serde_regex")]
    #[serde(default)] // If not supplied, serde supplies an empty vector.
    pub parent_folder_contains_regexp: Vec<Regex>,
}
