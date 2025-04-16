// src/debug/debug.rs
// github.com/cvusmo/hyprclock

use crate::configuration::logger::{log_debug, log_error, log_info, AppState};
use std::sync::{Arc, Mutex};

pub fn enable_debug_mode(state: &Arc<Mutex<AppState>>) -> Result<bool, String> {
    log_debug(state, "Debug mode is ON");
    let debug_success = initialize_debugging_actions(state);

    if debug_success {
        // Return successful
        log_info(state, "Debug mode setup complete.");
        Ok(true)
    } else {
        // Return false
        let error_message = "Failed to initialize debugging actions.".to_string();
        log_error(state, &error_message);
        eprintln!("{}", error_message);
        Err(error_message)
    }
}

// Initialize debug logic tasks
fn initialize_debugging_actions(state: &Arc<Mutex<AppState>>) -> bool {
    log_info(state, "Initializing additional debug features...");
    // TODO: add debug logic

    true
}
