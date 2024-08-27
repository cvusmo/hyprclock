// src/gui/window
// github.com/cvusmo/hyprclock

use chrono::{DateTime, Local};
use gio::Settings;
use glib::ControlFlow::Continue;
use gtk4 as gtk;
use gtk::{prelude::*, gio, 
        Application, ApplicationWindow, Grid, 
        Label, Switch, CssProvider, gdk::Display};
use std::path::Path;

use crate::configuration::animation::AnimationConfig;

const APP_ID: &str = "org.cvusmo.Hyprclock";

pub fn build_ui(app: &Application) -> ApplicationWindow {

    let settings = Settings::new(APP_ID);

    // BEGIN CONFIGURATION
    let animation_config = AnimationConfig::new();
    let (blur_enabled, fade_in_enabled) = animation_config.animation_default_settings();

    // LOAD STYLE.css
    let provider = CssProvider::new();
    provider.load_from_path(Path::new("style.css"));
    // TODO: add LOGGER for error, debug, info

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let gtk_settings = gtk::Settings::default().unwrap();
    // END OF CONFIGURATION

    // SWITCH DARK MODE TO LIGHT MODE
    let is_switch_enabled = settings.boolean("is-switch-enabled");

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

        eprintln!("Blur enabled: {}", blur_enabled);
        eprintln!("Fade-in enabled: {}", fade_in_enabled);
        
        glib::Propagation::Proceed
    });

    // PLACEHOLDER FOR HYPRCLOCK
    let clock_label = Label::builder()
        .label(get_current_time())
        .build();

    // APPLY DEFAULT CONFIG 

    // create 3x4 grid for window
    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    grid.attach(&switch, 0, 0, 1, 1);
    grid.attach(&clock_label, 1, 1, 2, 2);

    // builds window 
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&grid)
        .build();

    std::time::Duration::from_secs(1);
    glib::timeout_add_seconds_local(1, move || {
        clock_label.set_label(&get_current_time());
        Continue
    });

    window    
}

fn get_current_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
