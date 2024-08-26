use gio::Settings;
use chrono::{DateTime, Local};
use async_std::task;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, Align, Application, ApplicationWindow, Grid, Label, Switch};
use gtk::CssProvider;
use gtk::gdk::Display;
use std::path::Path;
use std::time::Duration;

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| build_ui(app));
    app.run()
}

fn build_ui(app: &Application) {
    let settings = Settings::new(APP_ID);
    let is_switch_enabled = settings.boolean("is-switch-enabled");

    let provider = CssProvider::new();

    // Try to load the CSS file from the specified path
    provider.load_from_path(Path::new("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );


    let gtk_settings = gtk::Settings::default().unwrap();

    // Create the switch for toggling themes
    let switch = Switch::builder()
        .state(is_switch_enabled)
        .build();

    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");

        let new_theme = if is_enabled {
            Some("Materia-dark")  // Example dark theme
        } else {
            Some("Materia")  // Example light theme
        };

        gtk_settings.set_gtk_theme_name(new_theme);
        
        glib::Propagation::Proceed
    });

    // Create a Grid container for a 3x4 layout
    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    // Create a label for the clock
    let clock_label = Label::builder()
        .label(&get_current_time())
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    // Add the switch to the top-left corner (position 0, 0)
    grid.attach(&switch, 0, 0, 1, 1);
    
    // Attach the clock label to span across the center cells (1, 1) to (2, 2)
    grid.attach(&clock_label, 1, 1, 2, 2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&grid)
        .build();

    window.present();
}

fn get_current_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
