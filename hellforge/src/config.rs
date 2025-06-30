use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub server: String,
    pub mode: String,
    pub interval_in_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: "localhost:8080".to_string(),
            mode: "instant".to_string(),
            interval_in_seconds: 69,
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("hellforge/config.json")
}

pub fn load_config() -> Option<Config> {
    let path = config_path();
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_config(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, serde_json::to_string_pretty(config)?)
}
