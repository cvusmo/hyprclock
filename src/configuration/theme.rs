// ~/cvusmo/hyprclock/src/configuration/theme.rs

use crate::configuration::logger::{log_info, AppState};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub background_color: String,
    pub font_color: String,
    pub font_size: f32,
    pub scale_factor: f32,
}

impl ThemeConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

// ThemeConfig
impl ThemeConfig {
    // Function to validate
    pub fn validate(&self) -> Result<(), String> {
        // Validate background color format
        if !self.background_color.starts_with('#') || self.background_color.len() != 7 {
            return Err("Invalid background color format. Must be a hex color code.".to_string());
        }

        // Validate font color format
        if !self.font_color.starts_with('#') || self.font_color.len() != 7 {
            return Err("Invalid text color format. Must be a hex color code.".to_string());
        }

        // Ensure font size is within a reasonable range
        if self.font_size <= 0.0 || self.font_size > 72.0 {
            return Err(
                "Font size must be greater than 0 and less than or equal to 72.".to_string(),
            );
        }

        // Ensure scale factor is within range 0.0 to 1.0
        if self.scale_factor <= 0.0 || self.scale_factor > 1.0 {
            return Err("Scale factor must be greater than 0 and no more than 1.0.".to_string());
        }

        Ok(())
    }

    // Function to Load Theme
    pub fn load_theme(&self, state: &Arc<Mutex<AppState>>) -> (String, String, f32, f32) {
        log_info(
            state,
            &format!("Background color: {}", self.background_color),
        );
        log_info(state, &format!("Font color: {}", self.font_color));
        log_info(state, &format!("Font size: {}", self.font_size));
        log_info(state, &format!("Scale factor: {}", self.scale_factor));
        (
            self.background_color.clone(),
            self.font_color.clone(),
            self.font_size,
            self.scale_factor,
        )
    }
}

// Default ThemeConfig
impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            background_color: "#1C1B1A".to_string(),
            font_color: "#F4E3C1".to_string(),
            font_size: 72.0,
            scale_factor: 1.0,
        }
    }
}
