use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application};

mod window;

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(run_main);
    app.run()
}

fn run_main(app: &Application) {
    let window = window::build_ui(app);
    window.present();
}
