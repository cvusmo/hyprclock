//src/main.rs
//github.com/cvusmo/hyprclock

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
    
    // initialize config and update
    let _check = Config::check_config();
    let config = Config::new(); 
    let _update = config.update();

    // initialize window and build the ui
    let window = gui::window::build_ui(app);
    window.present();
}
