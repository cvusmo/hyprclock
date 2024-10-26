// src/configuration/general.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub clock_format: String,
}

impl GeneralConfig {
    pub fn new() -> Self {
        GeneralConfig {
            clock_format: String::from("24-hour"), // Default value
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let valid_formats = vec!["12-hour", "24-hour"];
        if valid_formats.contains(&self.clock_format.as_str()) {
            Ok(())
        } else {
            Err(format!("Invalid clock format: {}", self.clock_format))
        }
    }
}

