// src/configuration/general.rs
// github.com/cvusmo/hyprclock

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub clock_format: String,   // 12-hour or 24-hour
    pub time_precision: String, // short or long
}

impl GeneralConfig {
    pub fn new() -> Self {
        GeneralConfig {
            clock_format: String::from("24-hour"), // Default value
            time_precision: String::from("long"),  // Default value
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // TODO: fix useless use of vec!
        let valid_formats = vec!["12-hour", "24-hour"];
        let valid_precisions = vec!["short", "long"];
        let mut errors = Vec::new();

        // Validation checks
        if !valid_formats.contains(&self.clock_format.as_str()) {
            errors.push(format!("Invalid clock format: {}", self.clock_format));
        }

        if !valid_precisions.contains(&self.time_precision.as_str()) {
            errors.push(format!("Invalid time precision: {}", self.time_precision));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}
