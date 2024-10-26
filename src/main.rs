// src/main.rs
// github.com/cvusmo/hyprclock

mod configuration;
mod gui;
mod debug;

use crate::configuration::{
    config::Config,
    logger::{create_state, log_error, log_info, log_warn, setup_logging, AppState},
};

use crate::debug::debug::enable_debug_mode;

use clap::{Arg, Command};
use gtk::{glib, prelude::*, Application};
use gtk4 as gtk;
use std::sync::{Arc, Mutex};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let _gtkinit = gtk::init();

    let matches = Command::new("hyprclock")
        .version("0.1.0a")
        .about("Hyprclock - A clock widget for time wizards")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enables debug mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Specifies a custom config file")
                .value_name("FILE")
                .num_args(1),
        )
        .get_matches();

    // --debug flag
    let debug_mode = *matches.get_one::<bool>("debug").unwrap_or(&false);
    if debug_mode {
        enable_debug_mode();
    }

    // Create log
    let state = create_state();
    if let Err(e) = setup_logging(&state, debug_mode) {
        log_error(&state, &format!("Failed to setup logging: {}", e));
    }

    // Handle config file
    let config_file = matches.get_one::<String>("config").cloned(); 
    if let Some(file) = &config_file {
        log_info(&state, &format!("Using config file: {}", file));
    }

    // Create application
    let app = Application::builder().application_id(APP_ID).build();
    
    // Pass the config_file to run_main
    app.connect_activate(move |app| run_main(app, &state, config_file.clone()));
    app.run()
}

fn run_main(app: &Application, state: &Arc<Mutex<AppState>>, config_file: Option<String>) {
    // Initialize config
    let config = match Config::check_config(config_file) { 
        // Pass config_file to check_config
        Ok(config) => config,
        Err(e) => {
            log_error(state, &format!("Failed to load config: {}", e));
            log_warn(state, "Using default configuration due to error.");
            log_info(state, &format!("Logging info check: {}", e));
            Config::new() 
        }
    };

    // Initialize window
    let window = gui::window::build_ui(app, &config, state);
    window.present();
}
