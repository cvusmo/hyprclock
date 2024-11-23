// src/logger.rs

use fern::Dispatch;
use gtk::Label;
use gtk4 as gtk;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::{env, error::Error, fs::File};

static LOGGER_INITIALIZED: OnceCell<bool> = OnceCell::new();

pub struct AppState {
    pub log_label: Label,
}

// General initialization function for both normal and debug modes
fn initialize_logger(
    state: &Arc<Mutex<AppState>>,
    log_file_path: &str,
    log_level: log::LevelFilter,
) -> Result<(), Box<dyn Error>> {
    if LOGGER_INITIALIZED.get().is_some() {
        log_info(state, "Logger is already initialized.");
        return Ok(());
    }

    let log_file_result = File::create(log_file_path)?;

    Dispatch::new()
        .format(|out, message, record| {
            let module = record.target().split("::").last().unwrap_or("unknown");
            let line = record
                .line()
                .map_or("unknown".to_string(), |l| l.to_string());
            out.finish(format_args!(
                "[{}] {}, {}:{}",
                record.level(),
                message,
                module,
                line
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(log_file_result)
        .apply()?;

    LOGGER_INITIALIZED.set(true).unwrap();
    log_info(
        state,
        &format!("Logger successfully created: {}", log_file_path),
    );
    Ok(())
}

// Setup logging
pub fn setup_logging(state: &Arc<Mutex<AppState>>, debug: bool) -> Result<(), Box<dyn Error>> {
    log_info(state, "Setting up logging...");

    let user_dir = env::var("HOME").unwrap_or_else(|_| "/home/default".to_string());
    let log_file_path = if debug {
        format!("{}/.config/hypr/hyprclock-debug.log", user_dir)
    } else {
        format!("{}/.config/hypr/hyprclock.log", user_dir)
    };

    let log_level = if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    initialize_logger(state, &log_file_path, log_level)
}

// Create state
pub fn create_state() -> Arc<Mutex<AppState>> {
    let log_label = Label::new(None);
    Arc::new(Mutex::new(AppState { log_label }))
}

// Utility functions for logging
pub fn update_log_label(state: &Arc<Mutex<AppState>>, message: &str) {
    let state = state.lock().unwrap();
    state.log_label.set_label(message);
}

pub fn log_info(state: &Arc<Mutex<AppState>>, message: &str) {
    log::info!("{}", message);
    update_log_label(state, message);
}

pub fn log_debug(state: &Arc<Mutex<AppState>>, message: &str) {
    log::debug!("{}", message);
    update_log_label(state, message);
}

pub fn log_warn(state: &Arc<Mutex<AppState>>, message: &str) {
    log::warn!("{}", message);
    update_log_label(state, message);
}

pub fn log_error(state: &Arc<Mutex<AppState>>, message: &str) {
    log::error!("{}", message);
    update_log_label(state, message);
}
