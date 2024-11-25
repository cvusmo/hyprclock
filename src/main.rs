mod configuration;
mod debug;
mod gui;

use crate::configuration::{
    config::Config,
    flags::Args,
    logger::{create_state, log_debug, log_error, log_info, setup_logging},
};
use crate::debug::debug::enable_debug_mode;
use crate::gui::window::build_ui;

use gtk::{glib, prelude::*, Application};
use gtk4 as gtk;
use std::sync::Arc;

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    // Initialize GTK
    let _gtk_init = gtk4::init();

    // Parse command line arguments
    let args = Args::parse_and_validate();

    // Create state
    let state = create_state();

    let debug_mode = args.debug;
    let log_mode = args.log;

    // Setup logging
    if let Err(err) = setup_logging(&state, debug_mode) {
        eprintln!("Failed to setup logging: {}", err);
        return glib::ExitCode::FAILURE;
    }

    // Handle debug mode
    if debug_mode {
        log_debug(&state, "Debug mode flag set, initializing debug mode...");
        if let Err(err) = enable_debug_mode(&state) {
            log_error(&state, &format!("Failed to enable debug mode: {}", err));
            return glib::ExitCode::FAILURE;
        }
        log_info(&state, "Running in debug mode...");
    }

    // Log mode
    if log_mode {
        eprintln!("Log mode enabled.");
        log_info(&state, "Log mode is enabled. Log is located at: {}");
        // TODO: need to create path to print the log location
    }

    // Load configuration
    let config = match Config::load_config(args.config) {
        Ok(cfg) => cfg,
        Err(err) => {
            log_error(&state, &format!("Failed to load configuration: {}", err));
            Config::new() // Fallback to default configuration
        }
    };

    // Create and run the GTK application
    create_and_run_app(&state, config, args.debug)
}

// Function to create and run the GTK application
fn create_and_run_app(
    state: &Arc<std::sync::Mutex<configuration::logger::AppState>>,
    config: Config,
    debug_mode: bool,
) -> glib::ExitCode {
    log_info(state, "Creating application...");

    // Create application
    let app = Application::builder().application_id(APP_ID).build();

    // Clone state
    let state_clone = Arc::clone(state);

    // Connect, activate, and initialize UI
    app.connect_activate(move |app| {
        log_info(&state_clone, "Application activated...");

        // Initialize window with debug_mode
        log_info(&state_clone, "Building the main UI...");
        let window = build_ui(app, &config, &state_clone, debug_mode);
        window.present();
    });

    // Run the application
    log_info(state, "Running the application...");
    app.run();
    glib::ExitCode::SUCCESS
}
