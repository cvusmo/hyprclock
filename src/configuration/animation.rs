// src/configuration/animation.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimationConfig {
    pub name: String, 
    pub blur_enabled: bool,
    pub fade_in_enabled: bool,
}

impl AnimationConfig {
    pub fn new(name: &str) -> Self {
        AnimationConfig {
            name: name.to_string(),
            blur_enabled: true,    // Default value
            fade_in_enabled: true, // Default value
        }
    }

    pub fn animation_default_settings(&self) -> (bool, bool) {
        (self.blur_enabled, self.fade_in_enabled)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.blur_enabled && self.fade_in_enabled {
            // Custom logic for conflicting or valid settings
            Ok(())
        } else {
            Err("Invalid animation configuration.".to_string())
        }
    }
}
