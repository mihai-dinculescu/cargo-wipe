use super::{config::Config, language_option::LanguageOption};
use directories::BaseDirs;
use std::{collections::HashMap, path::PathBuf};

pub fn read_from_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    let config_file = std::fs::read_to_string(path)?;

    //TODO: Need to catch errors deserializing RegEx entries! How should we fail? Soft or hard?
    let option_map: HashMap<String, LanguageOption> = serde_json::from_str(&config_file)?;

    let config: Config = option_map
        .into_iter()
        .map(|(k, v)| {
            let mut v = v;
            v.option_name = k.clone();
            (k, v)
        })
        .collect::<HashMap<String, LanguageOption>>()
        .into();

    Ok(config)
}

pub fn save_to_config(path: PathBuf, config: &Config) -> Result<(), anyhow::Error> {
    let config_file =
        serde_json::to_string_pretty::<HashMap<String, LanguageOption>>(&(config.into()))?;
    std::fs::write(path, config_file)?;

    Ok(())
}

pub fn get_config_path() -> Result<PathBuf, anyhow::Error> {
    let config_dir = BaseDirs::new()
        .ok_or(anyhow::anyhow!(
            "Could not get base directories, can not load configuration!"
        ))?
        .config_dir()
        .join("cargo-wipe");

    std::fs::create_dir_all(&config_dir)?;

    let config_filepath = config_dir.join("config.json");

    Ok(config_filepath)
}

pub fn save_config(config: &Config) -> Result<(), anyhow::Error> {
    let config_path = get_config_path()?;

    save_to_config(config_path, config)
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    let config_path = get_config_path()?;

    read_from_config(config_path)
}
