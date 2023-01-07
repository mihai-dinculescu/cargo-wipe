use super::language_option::LanguageOption;
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};

// Should we ditch this config struct in favor of a simple hash map itself?
#[derive(Debug, Clone)]
pub struct Config {
    // Not pub because it should not be mutable to the program
    language_options: HashMap<String, LanguageOption>,
}

impl Config {
    pub fn get_all_options(&self) -> impl Iterator<Item = &LanguageOption> {
        self.language_options.iter().map(|(_, v)| v) // To clone or not to clone? That is the question.
    }

    pub fn get_option(&self, option_name: &str) -> Option<&LanguageOption> {
        self.language_options.get(option_name)
    }

    pub fn has_option(&self, option_name: &str) -> bool {
        self.language_options.contains_key(option_name)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut language_options: HashMap<String, LanguageOption> = HashMap::new();

        let rust_language_option = LanguageOption {
            option_name: "rust".to_string(),
            target_folder_regexp: Regex::new(r"target").unwrap(),
            target_folder_contains_regexp: Vec::default(),
            parent_folder_contains_regexp: Vec::default(),
        };

        let node_language_option = LanguageOption {
            option_name: "node".to_string(),
            target_folder_regexp: Regex::new(r"node_modules").unwrap(),
            target_folder_contains_regexp: Vec::default(),
            parent_folder_contains_regexp: vec![Regex::new(r"package\.json").unwrap()],
        };

        language_options.insert(
            rust_language_option.option_name.clone(),
            rust_language_option,
        );

        language_options.insert(
            node_language_option.option_name.clone(),
            node_language_option,
        );

        Config { language_options }
    }
}

impl From<Config> for HashMap<String, LanguageOption> {
    fn from(value: Config) -> Self {
        value.language_options
    }
}

impl From<&Config> for HashMap<String, LanguageOption> {
    fn from(value: &Config) -> Self {
        value.language_options.clone()
    }
}

impl From<HashMap<String, LanguageOption>> for Config {
    fn from(value: HashMap<String, LanguageOption>) -> Self {
        Config {
            language_options: value,
        }
    }
}
