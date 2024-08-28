// src/main.rs
// github.com/cvusmo/hyprclock

mod configuration;
mod gui;

use crate::configuration::{
    config::Config,
    logger::{create_state, log_error, log_info, log_warn, setup_logging, AppState},
};
use gtk::{glib, prelude::*, Application};
use gtk4 as gtk;
use std::sync::{Arc, Mutex};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let _gtkinit = gtk::init();

    // Setup logging
    setup_logging().expect("Failed to setup logging");
    log_test();

    // Create application
    let app = Application::builder().application_id(APP_ID).build();
    let state = create_state();

    app.connect_activate(move |app| run_main(app, &state));
    app.run()
}

fn run_main(app: &Application, state: &Arc<Mutex<AppState>>) {
    // Initialize config
    let config = match Config::check_config() {
        Ok(config) => config,
        Err(e) => {
            log_error(state, &format!("Failed to load config: {}", e));
            log_warn(state, &format!("WARN TEST: {}", e));
            log_info(state, &format!("INFO TEST: {}", e));
            Config::new()
        }
    };

    // Initialize window
    let window = gui::window::build_ui(app, &config, state);
    window.present();
}

fn log_test() {
    let state = create_state();

    // Log test messages
    log_info(&state, "INFO test message");
    log_warn(&state, "WARN test message");
    log_error(&state, "ERROR test message");
}
