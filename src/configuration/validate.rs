// src/configuration/validate.rs
// github.com/cvusmo/hyprclock

use std::collections::HashSet;
use crate::configuration::{animation::AnimationConfig, theme::ThemeConfig};

// Validation function for animations
pub fn validate_animations(animations: &[AnimationConfig]) -> Result<(), String> {
    let mut active_animations = HashSet::new();

    for animation in animations {
        if animation.blur && !active_animations.insert("blur") {
            let error_message = "Error: 'blur' is set to true multiple times.".to_string();
            // Log the error somewhere else if needed, or handle it directly
            return Err(error_message);
        }

        if animation.fade_in && !active_animations.insert("fade_in") {
            let error_message = "Error: 'fade_in' is set to true multiple times.".to_string();
            // Log the error somewhere else if needed, or handle it directly
            return Err(error_message);
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
    // Call validate method from theme.rs
    theme.validate()
        .map_err(|e| format!("Theme validation failed: {}", e))
}

// General validation function
pub fn validate_all(animations: &[AnimationConfig], theme: &ThemeConfig) -> Result<(), String> {
    // Validate animations first
    validate_animations(animations)?;

    // Validate theme configuration
    validate_theme(theme)?;

    Ok(())
}

