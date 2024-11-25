// src/configuration/validate.rs
// github.com/cvusmo/hyprclock

use crate::configuration::{
    animation::AnimationConfig, env::EnvConfig, general::GeneralConfig, theme::ThemeConfig,
};
use std::collections::HashSet;

// Validation function for animations
pub fn validate_animations(animations: &[AnimationConfig]) -> Result<(), String> {
    let mut active_animations = HashSet::new();

    for animation in animations {
        if animation.blur && !active_animations.insert("blur") {
            return Err("Error: 'blur' is set to true multiple times.".to_string());
        }

        if animation.fade_in && !active_animations.insert("fade_in") {
            return Err("Error: 'fade_in' is set to true multiple times.".to_string());
        }
    }

    // Conflict checks for blur and fade_in
    let has_blur_enabled = animations.iter().any(|a| a.blur);
    let has_blur_disabled = animations.iter().any(|a| !a.blur);
    if has_blur_enabled && has_blur_disabled {
        return Err("Error: 'blur' animation cannot be both true and false.".to_string());
    }

    let has_fade_in_enabled = animations.iter().any(|a| a.fade_in);
    let has_fade_in_disabled = animations.iter().any(|a| !a.fade_in);
    if has_fade_in_enabled && has_fade_in_disabled {
        return Err("Error: 'fade_in' animation cannot be both true and false.".to_string());
    }

    Ok(())
}

// Validation function for theme
pub fn validate_theme(theme: &ThemeConfig) -> Result<(), String> {
    theme
        .validate()
        .map_err(|e| format!("Theme validation failed: {}", e))
}

// Validation function for general configuration
pub fn validate_general(general: &GeneralConfig) -> Result<(), String> {
    general
        .validate()
        .map_err(|e| format!("General configuration validation failed: {}", e))
}

// Validation function for environment configuration
pub fn validate_environment(env: &EnvConfig) -> Result<(), String> {
    env.validate()
        .map_err(|e| format!("Environment configuration validation failed: {}", e))
}
