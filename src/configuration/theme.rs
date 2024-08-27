// src/configuration/theme.rs
// github.com/cvusmo/hyprclock

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub theme_name: String,
}

impl ThemeConfig {
    pub fn new() -> Self {
        ThemeConfig {
            theme_name: String::from("Materia-dark"), // Default value
        }
    }
}
