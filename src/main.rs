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

use clap::{ArgAction, Parser};
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
    /// Run the application in debug mode
    #[arg(long, action = ArgAction::SetTrue)]
    debug: bool,

    /// Run the application in normal mode
    #[arg(long, action = ArgAction::SetTrue)]
    normal: bool,

    /// Specify the config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
}

fn main() -> glib::ExitCode {
    // Initialize GTK
    let _gtk_init = gtk4::init();

    // Create state
    let state = create_state();

    // Parse command line arguments
    let args = Args::parse();

    // Print the parsed arguments for debugging
    println!("Parsed Arguments: {:?}", args);

    // Determine modes
    let debug_mode = args.debug; // let mut debug_mode
    let normal_mode = args.normal;
    let config_file = args.config;

    // Debug print parsed modes
    println!("debug_mode: {}, normal_mode: {}", debug_mode, normal_mode);

    if debug_mode && normal_mode {
        eprintln!("Cannot run in both debug and normal mode. Please choose one.");
        return glib::ExitCode::FAILURE;
    }

    // Handle debug mode
    if debug_mode {
        log_info(&state, "Debug mode flag set, initializing debug mode...");
        if let Err(err) = setup_logging(&state, debug_mode) {
            eprintln!("Failed to setup logging: {}", err);
            return glib::ExitCode::FAILURE;
        }

        // Enable debug mode
        match enable_debug_mode(&state) {
            Ok(true) => {
                // debug_mode = true;
                log_info(&state, "Debug flag set to true");
            }
            Err(err) => {
                eprintln!("Failed to enable debug mode: {}", err);
                return glib::ExitCode::FAILURE;
            }
            _ => {
                eprintln!("Unknown error when attempting to enable debug mode.");
                return glib::ExitCode::FAILURE;
            }
        }

        log_info(&state, "Running in debug mode...");
        create_and_run_app(&state, config_file, true);
        return glib::ExitCode::SUCCESS;
    }

    // Handle normal mode
    if normal_mode {
        log_info(&state, "Normal mode flag set, initializing normal mode...");
        create_and_run_app(&state, config_file, false);
        return glib::ExitCode::SUCCESS;
    }

    // Default mode
    log_info(&state, "Starting hyprclock in default mode...");
    create_and_run_app(&state, config_file, false);

    glib::ExitCode::SUCCESS
}

// Function to create and run the GTK application
fn create_and_run_app(
    state: &Arc<std::sync::Mutex<configuration::logger::AppState>>,
    config_file: Option<String>,
    debug_mode: bool,
) {
    log_info(state, "Creating application...");

    // Create application
    let app = Application::builder().application_id(APP_ID).build();

    // Clone state
    let state_clone = Arc::clone(state);
    let config_file_clone = config_file.clone();

    // Connect, activate, and initialize UI
    app.connect_activate(move |app| {
        log_info(&state_clone, "Application activated...");

        // Initialize config
        let config = match Config::check_config(config_file_clone.clone()) {
            Ok(config) => config,
            Err(err) => {
                log_error(&state_clone, &format!("Failed to load config: {}", err));
                Config::new()
            }
        };

        // Initialize window with debug_mode
        log_info(&state_clone, "Building the main UI...");
        let window = build_ui(app, &config, &state_clone, debug_mode);
        window.present();
    });

    // Run the application
    log_info(state, "Running the application...");
    app.run();
}

