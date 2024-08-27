// src/configuration/general.rs
// github.com/cvusmo/hyprclock

use serde::{Serialize, Deserialize};

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
}
