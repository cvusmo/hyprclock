//src/main.rs
//github.com/cvusmo/hyprclock

mod gui;
mod configuration;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(run_main);
    app.run()
}

fn run_main(app: &Application) {
    let window = gui::window::build_ui(app);
    window.present();
}
