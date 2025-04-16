use clap::Parser;
use gtk4::{glib, prelude::*, Application};
use hyprclock::configuration::{
    config::Config,
    flags::Args,
    logger::{create_state, log_debug, log_error, log_info, setup_logging, AppState},
};
use hyprclock::debug::debug::enable_debug_mode;
use hyprclock::gui::calendar::CalendarModule;
use hyprclock::gui::clock::ClockModule;
use hyprclock::gui::window::build_ui;
use serde_json::json;
use std::sync::{Arc, Mutex};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let _gtk_init = gtk4::init();
    let args = Args::parse(); // Use the clap-provided parse method.
    let state = create_state();

    if args.waybar {
        let config = Config::load_config(args.config).unwrap_or_else(|_| Config::new());
        let clock_module = ClockModule::new(&config, &state);
        // Build JSON using serde_json to properly escape any control characters.
        let output = json!({
            "text": clock_module.get_time(&config),
            "tooltip": CalendarModule::generate_tooltip(&clock_module)
        });
        println!("{}", output.to_string());
        return glib::ExitCode::SUCCESS;
    }

    if let Err(err) = setup_logging(&state, args.debug) {
        eprintln!("Failed to setup logging: {}", err);
        return glib::ExitCode::FAILURE;
    }

    if args.debug {
        log_debug(&state, "Debug mode flag set, initializing debug mode...");
        if let Err(err) = enable_debug_mode(&state) {
            log_error(&state, &format!("Failed to enable debug mode: {}", err));
            return glib::ExitCode::FAILURE;
        }
        log_info(&state, "Running in debug mode...");
    }

    if args.log {
        eprintln!("Log mode enabled.");
        log_info(&state, "Log mode is enabled. Log is located at: TODO");
    }

    let config = match Config::load_config(args.config) {
        Ok(cfg) => cfg,
        Err(err) => {
            log_error(&state, &format!("Failed to load configuration: {}", err));
            Config::new()
        }
    };

    create_and_run_app(&state, config, args.debug)
}

fn create_and_run_app(
    state: &Arc<Mutex<AppState>>,
    config: Config,
    debug_mode: bool,
) -> glib::ExitCode {
    log_info(state, "Creating application...");

    let app = Application::builder().application_id(APP_ID).build();
    let state_clone = Arc::clone(state);

    app.connect_activate(move |app| {
        log_info(&state_clone, "Application activated...");
        log_info(&state_clone, "Building the main UI...");
        let window = build_ui(app, &config, &state_clone, debug_mode);
        window.present();
    });

    log_info(state, "Running the application...");
    app.run();
    glib::ExitCode::SUCCESS
}
