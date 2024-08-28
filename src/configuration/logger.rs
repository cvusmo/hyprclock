//src/configuration/logger.rs
//github.com/cvusmo/hyprclock

use fern::Dispatch;
use gtk::Label;
use gtk4 as gtk;
use std::error::Error;
use std::fs::File;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub log_label: Label,
}

pub fn create_state() -> Arc<Mutex<AppState>> {
    let log_label = Label::new(None);
    Arc::new(Mutex::new(AppState { log_label }))
}

pub fn update_log_label(state: &Arc<Mutex<AppState>>, message: &str) {
    let state = state.lock().unwrap();
    state.log_label.set_label(message);
}

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    let log_file = File::create("hyprclock-debug.log")?;

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

    Ok(())
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
