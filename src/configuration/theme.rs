// src/configuration/theme.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub background_color: String,
    pub text_color: String,
    pub font_size: u32,
}

impl ThemeConfig {
    pub fn new() -> Self {
        Self {
            background_color: "#000000".to_string(), // Default values
            text_color: "#59F87E".to_string(),
            font_size: 14,
        }
    }
}
