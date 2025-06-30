use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};

#[derive(Serialize, Deserialize)]
pub struct SyncState {
    pub hashes: HashMap<String, String>,
}

impl SyncState {
    pub fn load() -> Self {
        fs::read_to_string(".sync_state.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| SyncState {
                hashes: HashMap::new(),
            })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(".sync_state.json", json)
    }

    pub fn calculate_hash(path: &str) -> std::io::Result<String> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 4096];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn should_upload(&mut self, path: &str) -> std::io::Result<bool> {
        let current_hash = Self::calculate_hash(path)?;
        match self.hashes.get(path) {
            Some(prev_hash) if prev_hash == &current_hash => Ok(false),
            _ => {
                self.hashes.insert(path.to_string(), current_hash);
                Ok(true)
            }
        }
    }
}
