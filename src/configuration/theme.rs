// src/configuration/theme.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub background_color: String,
    pub font_color: String,
    pub font_size: u32,
}

impl ThemeConfig {
    pub fn new() -> Self {
        Self::default() 
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            background_color: "#000000".to_string(),  // Default background
            font_color: "#59F87E".to_string(),  // Default font color
            font_size: 24,  // Default font size
        }
    }
}

impl ThemeConfig {
    pub fn validate(&self) -> Result<(), String> {
        // Validate background color format
        if !self.background_color.starts_with('#') || self.background_color.len() != 7 {
            return Err("Invalid background color format. Must be a hex color code.".to_string());
        }

        // Validate font color format
        if !self.font_color.starts_with('#') || self.font_color.len() != 7 {
            return Err("Invalid text color format. Must be a hex color code.".to_string());
        }

        // Ensure font size is greater than 0
        if self.font_size == 0 {
            return Err("Font size must be greater than 0.".to_string());
        }

        Ok(())
    }

    // Theme default settings if needed
    pub fn theme_default_settings(&self) -> (String, String, u32) {
        (
            self.background_color.clone(),
            self.font_color.clone(),
            self.font_size,
        )
    }
}
