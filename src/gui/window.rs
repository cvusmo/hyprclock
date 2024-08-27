// src/gui/window
// github.com/cvusmo/hyprclock

use gio::Settings;
use chrono::{DateTime, Local};
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, Align, Application, ApplicationWindow, Grid, Label, Switch};
use gtk::CssProvider;
use gtk::gdk::Display;
use std::path::Path;

const APP_ID: &str = "org.cvusmo.Hyprclock";

pub fn build_ui(app: &Application) -> ApplicationWindow {
    let settings = Settings::new(APP_ID);
    let is_switch_enabled = settings.boolean("is-switch-enabled");

    let provider = CssProvider::new();
    provider.load_from_path(Path::new("style.css"));
        //.expect("Failed to load CSS file");

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let gtk_settings = gtk::Settings::default().unwrap();

    // Create a switch that is always visible
    let switch = Switch::builder()
        .state(is_switch_enabled)
        .build();

    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");

        let new_theme = if is_enabled {
            eprintln!("DARK MODE");
            Some("Materia-dark") 
        } else {
            eprintln!("LIGHT MODE");
            Some("Materia")
        };

        gtk_settings.set_gtk_theme_name(new_theme);

        glib::Propagation::Proceed
    });
    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    let clock_label = Label::builder()
        .label(get_current_time())
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    grid.attach(&switch, 0, 0, 1, 1);
    grid.attach(&clock_label, 1, 1, 2, 2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&grid)
        .build();

    window    
}

fn get_current_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
