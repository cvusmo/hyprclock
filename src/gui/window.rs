// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::{
    configuration::logger::{log_debug, log_info, AppState},
    Config,
};
use glib::ControlFlow::Continue;
use gtk::{
    gdk::Display, prelude::*, Application, ApplicationWindow, CssProvider, Grid, Label, Switch,
};
use gtk4 as gtk;
use std::{env, path::Path, sync::Arc, sync::Mutex};

pub fn build_ui(
    app: &Application,
    config: &Config,
    state: &Arc<Mutex<AppState>>,
) -> ApplicationWindow {
   
    // Loading config...
    log_info(state, "Loading config...");

    // ENV
    // END ENV
    
    // GENERAL
    // END GENERAL

    // ANIMATION
    let (blur, fade_in) = config.animation.animation_default_settings();
    log_debug(state, &format!("Blur enabled: {}", blur));
    log_debug(state, &format!("Fade in enabled: {}", fade_in));
    // END ANIMATION

    // THEME
    // Load configuration safely (handle potential errors)
    let background_color = config.theme.background_color.as_str();
    log_info(state, &format!("Background color: {}", background_color));

    let font_color = config.theme.font_color.as_str();
    log_info(state, &format!("Font color: {}", font_color));

    let font_size = config.theme.font_size;
    log_info(state, &format!("Font size: {}", font_size));

    // Animation init
    let (blur, fade_in) = config.animation.animation_default_settings();
    log_debug(state, &format!("Blur enabled: {}", blur));
    log_debug(state, &format!("Fade in enabled: {}", fade_in));

    // Configuration dir path
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/$USER"));
    let config_file = format!("{}/.config/hypr/hyprclock.conf", home_dir);
    let config_path = Path::new(&config_file);
    log_info(state, &format!("Configuration file path: {}", config_path.display()),);

    // Generate CSS from the configuration
    let css = format!(
        "
        .clock {{
            color: {};
            font-size: {}px;
            width: 100%;
            height: 100%;
            text-align: center;
        }}
        .window {{
            background-color: {};
        }}
        ",
        font_color, 
        font_size, 
        background_color
    );

    let provider = CssProvider::new();
    provider.load_from_data(&css);

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    log_debug(state, &format!("Generated CSS:\n{}", css));
    log_info(state, "Building window...");

    // Attempt to build the window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .css_classes(vec!["window".to_string()])
        .build();

    // TODO: add switch for dark/light mode later
    //let switch = Switch::builder().build();
   
    let clock_label = Label::builder()
        .label(get_current_time())
        .css_classes(vec!["clock".to_string()])
        .build();

    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    //grid.attach(&switch, 0, 0, 1, 1);
    grid.attach(&clock_label, 0, 1, 2, 1);

    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true); 

    grid.set_halign(gtk::Align::Center); 
    grid.set_valign(gtk::Align::Center); 

    window.set_child(Some(&grid));

    // Time update (log any failures)
    glib::timeout_add_seconds_local(1, move || {
        let current_time = get_current_time();
        clock_label.set_label(&current_time); 
        
        Continue
    });

    log_info(state, "Window built successfully.");
    window
}

fn get_current_time() -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
