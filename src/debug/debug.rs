// src/debug/debug.rs
// github.com/cvusmo/hyprclock

use crate::configuration::logger::{log_debug, log_info, AppState};
use std::sync::{Arc, Mutex};

pub fn enable_debug_mode(state: &Arc<Mutex<AppState>>) -> Result<(), String> {
    log_debug(state, "Debug mode is ON");

    initialize_debugging_actions(state);

    log_info(state, "Debug mode setup complete.");
    Ok(())
}

// Initialize debug logic tasks
fn initialize_debugging_actions(state: &Arc<Mutex<AppState>>) {
    log_info(state, "Initializing additional debug features...");
    // TODO: add debug logic
}
