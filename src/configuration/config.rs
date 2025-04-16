use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{fs, io, path::PathBuf};
use toml;

pub use crate::configuration::animation::AnimationConfig;
pub use crate::configuration::env::EnvConfig;
pub use crate::configuration::general::GeneralConfig;
use crate::configuration::logger::AppState;
pub use crate::configuration::theme::ThemeConfig;
use crate::configuration::validate::{
    validate_animations, validate_environment, validate_general, validate_theme,
};

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

    /// Load or initialize configuration
    pub fn load_config(config_file: Option<String>) -> io::Result<Self> {
        let config_path = config_file
            .map(PathBuf::from)
            .unwrap_or_else(get_config_path);

        if config_path.exists() {
            let config_contents = fs::read_to_string(&config_path)?;
            let loaded_config: Self = toml::from_str(&config_contents)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            // Validate the loaded configuration using validate.rs
            Config::validate_config(&loaded_config)?;
            Ok(loaded_config)
        } else {
            // Create default config using load functions of each submodule
            // TODO: remove load and replace with new.
            // load function is only applicable IF there is a config file
            // saved. then load_config is responsible for loading THAT config
            // otherwise this else statement is to create a NEW animation, env
            // general, theme, to then save it as the default config because
            // *Config::new() IS the default setting
            let default_config = Config {
                animation: AnimationConfig::new(),
                env: EnvConfig::new(),
                general: GeneralConfig::new(),
                theme: ThemeConfig::new(),
            };
            default_config.save()?;
            Ok(default_config)
        }
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

    /// Validate the configuration using validate.rs
    fn validate_config(config: &Self) -> Result<(), io::Error> {
        let mut errors = Vec::new();

        if let Err(err) = validate_animations(&[config.animation.clone()]) {
            errors.push(err);
        }
        if let Err(err) = validate_general(&config.general) {
            errors.push(err);
        }
        if let Err(err) = validate_theme(&config.theme) {
            errors.push(err);
        }
        if let Err(err) = validate_environment(&config.env) {
            errors.push(err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Config validation failed: {:?}", errors),
            ))
        }
    }

    /// Load the theme settings
    pub fn load_theme(&self, state: &Arc<Mutex<AppState>>) -> (String, String, f32, f32) {
        self.theme.load_theme(state)
    }
}

/// Get the path to the configuration file
pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    path.push(".config/hypr/hyprclock.conf");
    path
}
