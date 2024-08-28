// src/main.rs
// github.com/cvusmo/hyprclock

mod configuration;
mod gui;

use crate::configuration::{
    config::Config,
    logger::{create_state, log_error, log_info, log_warn, AppState, setup_logging},
};
use clap::{Arg, Command};
use gtk::{glib, prelude::*, Application};
use gtk4 as gtk;
use std::sync::{Arc, Mutex};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let _gtkinit = gtk::init();

    let matches = Command::new("hyprclock")
        .about("Hyprclock - A clock application with optional debug mode")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enables debug mode")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Debug mode check
    let debug_mode = *matches.get_one::<bool>("debug").unwrap_or(&false);

    if let Err(e) = setup_logging(debug_mode) {
        eprintln!("Failed to set up logging: {}", e);
    }

    let state = create_state();

    // Create application
    let app = Application::builder().application_id(APP_ID).build();

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
