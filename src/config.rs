use std::{env, fs::File};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        if let Ok(path) = env::var("CHAT_APP_CONFIG") {
            let ret = serde_yaml::from_reader(File::open(path)?)?;
            Ok(ret)
        } else {
            let config_paths = vec![
                "app.yaml",
                "app.yml",
                "/ect/config/app.yaml",
                "/ect/config/app.yml",
            ];
            for path in config_paths {
                if let Ok(reader) = File::open(path) {
                    return Ok(serde_yaml::from_reader(reader)?);
                }
            }
            bail!("Config file not found")
        }
    }
}
