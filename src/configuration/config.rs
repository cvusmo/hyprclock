// src/configuration/config.rs
// github.com/cvusmo/hyprclock

use std::{io, path::PathBuf, fs};
use toml;
use serde::{Serialize, Deserialize};

pub use crate::configuration::animation::AnimationConfig;
pub use crate::configuration::env::EnvConfig;
pub use crate::configuration::general::GeneralConfig;
pub use crate::configuration::theme::ThemeConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub animation: AnimationConfig,
    pub env: EnvConfig,
    pub general: GeneralConfig,
    pub theme: ThemeConfig,
}

impl Config {
    // Default configuration
    pub fn new() -> Self {
        Config {
            animation: AnimationConfig::new(),
            env: EnvConfig::new(),
            general: GeneralConfig::new(),
            theme: ThemeConfig::new(),
        }
    }

    // Load configuration
    pub fn check_config() -> io::Result<Self> {
        let config_path = get_config_path();

        if !config_path.exists() {
            // Return default configuration
            let default_config = Config::new();
            default_config.update()?;
            return Ok(default_config);
        }

        // Read & parse configuration
        let config_contents = fs::read_to_string(config_path)?;
        let loaded_config: Self = toml::de::from_str(&config_contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(loaded_config)
    }

    // Save configuration
    pub fn update(&self) -> io::Result<()> {
        let config_path = get_config_path();

        // Create configuration dir
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write configuration
        let config_contents = toml::ser::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(config_path, config_contents)
    }
}

// Get configuration path
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    path.push(".config/hypr/hyprclock.conf");
    path
}
