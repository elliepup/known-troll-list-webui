use std::fs::File;
use std::io::{BufReader, Read};
use configparser::ini::Ini;
use std::env;

fn load_config_contents() -> String {
    let mut contents = String::new();
    BufReader::new(File::open("./src/resources/config.ini").expect("Config file does not exist"))
        .read_to_string(&mut contents)
        .expect("Config file is corrupt");
    contents
}

fn get_setting(section: &str, setting: &str) -> Result<String, String> {
    let contents = load_config_contents();
    let mut config = Ini::new();
    config.read(contents).map_err(|e| e.to_string())?;

    config.get(section, setting)
        .map(|value| value.to_string())
        .ok_or_else(|| format!("Setting {} not found in section {}", setting, section))
}

fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_env_from_config(section: &str, key: &str) -> Result<String, String> {
    let setting = get_setting(section, key)?;
    get_env(&setting[..]).ok_or_else(|| format!("Missing {} environment variable", key))
}