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
            font_size: 24,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.background_color.starts_with('#') || self.background_color.len() != 7 {
            return Err("Invalid background color format. Must be a hex color code.".to_string());
        }
        if !self.text_color.starts_with('#') || self.text_color.len() != 7 {
            return Err("Invalid text color format. Must be a hex color code.".to_string());
        }
        if self.font_size == 0 {
            return Err("Font size must be greater than 0.".to_string());
        }
        Ok(())
    }
}

