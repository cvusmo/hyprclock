// src/configuration/config.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use toml;

pub use crate::configuration::animation::AnimationConfig;
pub use crate::configuration::env::EnvConfig;
pub use crate::configuration::general::GeneralConfig;
pub use crate::configuration::theme::ThemeConfig;
use crate::configuration::validate::{validate_animations, validate_theme};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub animation: AnimationConfig,
    pub env: EnvConfig,
    pub general: GeneralConfig,
    pub theme: ThemeConfig,
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Config {
            animation: AnimationConfig::new(),
            env: EnvConfig::new(),
            general: GeneralConfig::new(),
            theme: ThemeConfig::new(),
        }
    }

    /// Build or load the configuration from file
    pub fn check_config(config_file: Option<String>) -> io::Result<Self> {
        let config_path = config_file
            .map(PathBuf::from)
            .unwrap_or_else(get_config_path);

        if !config_path.exists() {
            // Create and save the default configuration
            let default_config = Config::new();
            default_config.save()?;
            return Ok(default_config);
        }

        // Load and validate the configuration
        let config_contents = fs::read_to_string(&config_path)?;
        let loaded_config: Self = toml::from_str(&config_contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        loaded_config.validate().map_err(|errors| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Config validation failed: {:?}", errors),
            )
        })?;

        Ok(loaded_config)
    }

    /// Save the configuration to file
    pub fn save(&self) -> io::Result<()> {
        let config_path = get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_contents =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(config_path, config_contents)
    }

    /// Validate all submodule configurations
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate each submodule configuration
        if let Err(err) = validate_animations(&[self.animation.clone()]) {
            errors.push(err);
        }
        if let Err(err) = self.env.validate() {
            errors.push(err);
        }
        if let Err(err) = self.general.validate() {
            errors.push(err);
        }
        if let Err(err) = validate_theme(&self.theme) {
            errors.push(err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Get the path to the configuration file
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    path.push(".config/hypr/hyprclock.conf");
    path
}
