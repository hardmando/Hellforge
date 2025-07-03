use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub watched_path: String,
    pub server_ip: String,
    pub mode: String,
    pub interval_in_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watched_path: "./watched".into(),
            server_ip: "192.168.125".to_string(),
            mode: "instant".to_string(),
            interval_in_secs: 10,
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir().unwrap().join("hellforge/config.json")
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("hellforge_config.json")?;
    Ok(serde_json::from_str(&data)?)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(config)?;
    fs::write("hellforge_config.json", data)?;
    Ok(())
}
