use gio::Settings;
use chrono::Local;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, Align, Application, ApplicationWindow, Label, Orientation, Switch};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let settings = Settings::new(APP_ID);
    let is_switch_enabled = settings.boolean("is-switch-enabled");

//    let time = Local::now().format("%H:%M:%S").to_string();

    let switch = Switch::builder()
        .margin_top(3)
        .margin_bottom(3)
        .valign(Align::End)
        .halign(Align::Start)
        .state(is_switch_enabled)
        .build();
    
    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");
        glib::Propagation::Proceed
    });

    let clock_label = Label::builder()
        .label(&get_current_time())
        .halign(Align::Center)
        .valign(Align::Start)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&clock_label);
    gtk_box.append(&switch);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&gtk_box)
        .build();

    window.present();
}

fn get_current_time() -> String {
    let current_time = Local::now();
    current_time.format("%H:%M:%S").to_string() 
}
