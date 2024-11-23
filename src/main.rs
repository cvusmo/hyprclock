// src/main.rs
// github.com/cvusmo/hyprclock

mod configuration;
mod debug;
mod gui;

use crate::configuration::{
    config::Config,
    logger::{create_state, log_error, log_info, setup_logging},
};
use crate::debug::debug::enable_debug_mode;
use crate::gui::window::build_ui;

use clap::Parser;
use gtk::{glib, prelude::*, Application};
use gtk4 as gtk;
use std::sync::Arc;

const APP_ID: &str = "org.cvusmo.Hyprclock";

// Hyprclock - a clock widget for Time Wizards
#[derive(Parser, Debug)]
#[command(
    version = "0.1.1-100-p",
    about = "Hyprclock - a clock widget for Time Wizards"
)]
struct Args {
    // Setup modes
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    normal: bool,
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
}

fn main() -> glib::ExitCode {
    // Initialize gtk
    if let Err(err) = gtk4::init() {
        eprintln!("Failed to initialize GTK: {}", err);
        return glib::ExitCode::FAILURE;
    }

    // Create state
    let state = create_state();

    // Parse command line arguments
    let args = Args::parse();

    // Determine modes
    let debug_mode = args.debug;
    let normal_mode = args.normal;
    let config_file = args.config;

    // Setup logging based on mode
    if let Err(e) = setup_logging(&state, debug_mode) {
        log_error(&state, &format!("Failed to setup logging: {}", e));
        return glib::ExitCode::FAILURE;
    }

    // Handle config file
    if let Some(ref file) = config_file {
        log_info(&state, &format!("Using config file: {}", file));
    }

    // Enable debug mode if required
    if debug_mode {
        if let Err(err) = enable_debug_mode(&state) {
            log_error(&state, &format!("Failed to enable debug mode: {}", err));
            return glib::ExitCode::FAILURE;
        }
    }

    // Create application
    let app = Application::builder().application_id(APP_ID).build();

    // Clone necessary variables
    let state_clone = Arc::clone(&state);
    let config_file_clone = config_file.clone();

    // Connect activate and initialize the UI
    app.connect_activate(move |app| {
        // Initialize config
        let config = match Config::check_config(config_file_clone.clone()) {
            Ok(config) => config,
            Err(err) => {
                log_error(&state_clone, &format!("Failed to load config: {}", err));
                Config::new()
            }
        };

        // Initialize window with debug_mode awareness
        let window = build_ui(app, &config, &state_clone, args.debug);
        window.present();
    });

    // Run the application in debug or normal mode
    if debug_mode {
        log_info(&state, "Running in debug mode...");
    } else if normal_mode {
        log_info(&state, "Running in normal mode...");
    } else {
        log_info(&state, "Running in default mode...");
    }

    // Run the application
    app.run()
}
