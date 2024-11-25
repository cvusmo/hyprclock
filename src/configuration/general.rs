// src/configuration/general.rs
// github.com/cvusmo/hyprclock

use crate::configuration::logger::{log_info, AppState};
use chrono::{DateTime, Local};
use glib::ControlFlow::Continue;
use gtk4::Label;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub clock_format: String,   // 12-hour or 24-hour
    pub time_precision: String, // short or long
}

impl GeneralConfig {
    pub fn new() -> Self {
        GeneralConfig {
            clock_format: String::from("12-hour"), // Default value
            time_precision: String::from("long"),  // Default value
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let valid_formats = vec!["12-hour", "24-hour"];
        let valid_precisions = vec!["short", "long"];
        let mut errors = Vec::new();

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

    // Method to start updating the clock label every second
    pub fn start_clock_update(self, clock_label: Arc<Label>, state: Arc<Mutex<AppState>>) {
        let general_config = self.clone(); // Clone the GeneralConfig instance to move into the closure

        glib::timeout_add_seconds_local(1, move || {
            let current_time = general_config.get_current_time();
            clock_label.set_label(&current_time);
            log_info(&state, &format!("Updated clock label to: {}", current_time));
            Continue
        });
    }

    // Method to get the current time formatted according to settings
    pub fn get_current_time(&self) -> String {
        let now: DateTime<Local> = Local::now();
        match (self.clock_format.as_str(), self.time_precision.as_str()) {
            ("24-hour", "short") => now.format("%H:%M").to_string(),
            ("24-hour", "long") => now.format("%H:%M:%S").to_string(),
            ("12-hour", "short") => now.format("%I:%M %p").to_string(),
            ("12-hour", "long") => now.format("%I:%M:%S %p").to_string(),
            _ => now.format("%H:%M:%S").to_string(),
        }
    }
}
