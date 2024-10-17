// src/configuration/validate.rs
// github.com/cvusmo/hyprclock

use std::collections::HashSet;
use crate::configuration::animation::AnimationConfig;

pub fn validate_animations(animations: &[AnimationConfig]) -> Result<(), String> {
    let mut active_animations = HashSet::new();

    for animation in animations {
        // Check for active animations (combining conditions)
        if animation.blur && !active_animations.insert("blur") {
            return Err("Error: 'blur' is set to true multiple times.".to_string());
        }

        if animation.fade_in && !active_animations.insert("fade_in") {
            return Err("Error: 'fade_in' is set to true multiple times.".to_string());
        }
    }

    // Check for conflicting states: Each animation should not be both enabled and disabled
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
