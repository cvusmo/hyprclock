use fern::Dispatch;
use gtk::Label;
use gtk4 as gtk;
use std::error::Error;
use std::fs::File;
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

static LOGGER_INITIALIZED: OnceCell<bool> = OnceCell::new();

pub struct AppState {
    pub log_label: Label,
}

// Normal mode
pub fn normal_mode() -> Result<(), Box<dyn Error>> {
    if LOGGER_INITIALIZED.get().is_none() {
        println!("Creating log file..."); // Debugging output
        let log_file_result = File::create("hyprclock.log");

        match log_file_result {
            Ok(log_file) => {
                Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!(
                            "[{}][{}] {}",
                            record.level(),
                            record.target(),
                            message
                        ))
                    })
                    .level(log::LevelFilter::Info) 
                    .chain(std::io::stdout())
                    .chain(log_file)
                    .apply()?;

                LOGGER_INITIALIZED.set(true).ok();
                println!("Logger successfully created: hyprclock.log");

                // Test log message
                log::info!("Logger initialized in normal mode."); // This won't appear since level is Error
                log::warn!("This is a warning message."); // This won't appear since level is Error
                log::error!("This is an error message."); // This should appear in the log file
            }
            Err(e) => {
                eprintln!("Failed to create log file: {}", e);
                return Err(Box::new(e));
            }
        }
    } else {
        println!("Logger is already initialized.");
    }
    Ok(())
}

// DEBUG MODE
pub fn debug_mode() -> Result<(), Box<dyn Error>> {
    if LOGGER_INITIALIZED.get().is_none() {
        let log_file_result = File::create("hyprclock-debug.log");

        match log_file_result {
            Ok(log_file) => {
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

                // Print message indicating success
                println!("Logger successfully created: hyprclock-debug.log");
            }
            Err(e) => {
                eprintln!("Failed to create log file: {}", e);
                return Err(Box::new(e));
            }
        }
    } else {
        // Print message indicating logger was already initialized
        println!("Logger is already initialized.");
    }
    Ok(())
}


// Setup logging
pub fn setup_logging(debug: bool) -> Result<(), Box<dyn Error>> {
    if debug {
        debug_mode()
    } else {
        normal_mode()
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
