use fern::Dispatch;
use gtk::Label;
use gtk4 as gtk;
use std::error::Error;
use std::env;
use std::fs::File;
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

static LOGGER_INITIALIZED: OnceCell<bool> = OnceCell::new();

pub struct AppState {
    pub log_label: Label,
}

// Normal mode
pub fn normal_mode(state: &Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
  if LOGGER_INITIALIZED.get().is_none() {
        log_info(state, "Creating log file...");
        
        let user_dir = env::var("HOME").unwrap_or_else(|_| "/home/default".to_string());
        let log_file_path = format!("{}/.config/hypr/hyprclock.log", user_dir);
        let log_file_result = File::create(log_file_path);
        //let log_file_result = File::create("/home/$USER/.config/hypr/hyprclock.log");

        match log_file_result {
            Ok(log_file) => {
                // Simplified log format for normal mode
                Dispatch::new()
                    .format(|out, message, record| {
                        let module = record.target().split("::").last().unwrap_or("unknown");
                        let line = record.line().map_or("unknown".to_string(), |l| l.to_string());
                        out.finish(format_args!(
                            "[{}] {}, {}:{}",
                            record.level(),
                            message,
                            module,
                            line
                        ))
                    })
                    .level(log::LevelFilter::Info)
                    .chain(std::io::stdout())
                    .chain(log_file)
                    .apply()?;

                LOGGER_INITIALIZED.set(true).ok();
                log_info(state, "Logger successfully created: hyprclock.log");

                // Test log messages
                log_info(state, "Logger initialized in normal mode.");
                log_warn(state, "This is a warning message.");
                log_error(state, "This is an error message.");
            }
            Err(e) => {
                log_error(state, &format!("Failed to create log file: {}", e));
                return Err(Box::new(e));
            }
        }
    } else {
        log_info(state, "Logger is already initialized.");
    }
    Ok(())
}

// DEBUG MODE
pub fn debug_mode(state: &Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
    if LOGGER_INITIALIZED.get().is_none() {
        let log_file_result = File::create("hyprclock-debug.log");

        match log_file_result {
            Ok(log_file) => {
                // Detailed log format for debug mode
                Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!(
                            "[{}][{}] {}",
                            record.level(),
                            record.target(),
                            message
                        ))
                    })
                    .level(log::LevelFilter::Debug)
                    .chain(std::io::stdout())
                    .chain(log_file)
                    .apply()?;
                LOGGER_INITIALIZED.set(true).ok();

                log_info(state, "Logger successfully created: hyprclock-debug.log"); 
            }
            Err(e) => {
                log_error(state, &format!("Failed to create log file: {}", e)); 
                return Err(Box::new(e));
            }
        }
    } else {
        log_info(state, "Logger is already initialized."); 
    }
    Ok(())
}

// Setup logging
pub fn setup_logging(state: &Arc<Mutex<AppState>>, debug: bool) -> Result<(), Box<dyn Error>> {
    log_info(state, "Setting up logging...");
    if debug {
        debug_mode(state)
    } else {
        normal_mode(state)
    }
}

pub fn create_state() -> Arc<Mutex<AppState>> {
    let log_label = Label::new(None);
    Arc::new(Mutex::new(AppState { log_label }))
}

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

