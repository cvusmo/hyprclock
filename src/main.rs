// src/main.rs
// github.com/cvusmo/hyprclock

mod configuration;
mod gui;

use gtk4 as gtk;
use gtk::{prelude::*, glib, Application};
use crate::configuration::config::Config;

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(run_main);
    app.run()
}

fn run_main(app: &Application) {
    // Initialize config and update
    let config = match Config::check_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            Config::new()
        }
    };

    // Initialize window and build the UI
    let window = gui::window::build_ui(app, &config);
    window.present();
}
