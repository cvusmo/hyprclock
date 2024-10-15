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
    // Load configuration
    let background_color = config.theme.background_color.as_str();
    let text_color = config.theme.text_color.as_str();
    let font_size = config.theme.font_size;

    let css = format!(
        "
        .clock {{
            color: {};
            font-size: {}px;
        }}
        .window {{
            background-color: {};
        }}
        ",
        text_color, font_size, background_color
    );

    let clock_label = Label::builder().label(get_current_time()).build();

    // Animation init
    let (blur_enabled, fade_in_enabled) = config.animation.animation_default_settings();
    log_debug(state, &format!("Blur enabled: {}", blur_enabled));
    log_debug(state, &format!("Fade in enabled: {}", fade_in_enabled));

    // Configuration dir path
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/$USER"));
    let config_file = format!("{}/.config/hypr/hyprclock.conf", home_dir);
    let config_path = Path::new(&config_file);

    log_info(
        state,
        &format!("Configuration file path: {}", config_path.display()),
    );

    // Applies style
    let provider = CssProvider::new();
    provider.load_from_data(&css);

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    log_debug(state, &format!("Generated CSS:\n{}", css));

    // Dark/Light mode switch
    let switch = Switch::builder().build();

    // 3x4 grid for window
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    grid.attach(&switch, 0, 0, 1, 1);
    grid.attach(&clock_label, 1, 1, 2, 2);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&grid)
        .build();

    let _update = std::time::Duration::from_secs(1);
    glib::timeout_add_seconds_local(1, move || {
        clock_label.set_label(&get_current_time());
        Continue
    });

    window
}

fn get_current_time() -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
