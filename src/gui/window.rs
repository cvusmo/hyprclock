// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::{
    configuration::logger::{log_debug, log_info, AppState},
    Config,
};
use glib::ControlFlow::Continue;
use gtk::{
    gdk::Display, prelude::*, Application, ApplicationWindow, CssProvider, Grid, Label,
};
use gtk4 as gtk;
use std::{env, path::{Path, PathBuf}, sync::{Arc, Mutex}};

pub fn build_ui(
    app: &Application,
    config: &Config,
    state: &Arc<Mutex<AppState>>,
) -> ApplicationWindow {
    log_info(state, "Loading config...");
    
    let (background_color, font_color, font_size) = load_theme(config, state);
    let _config_path = load_configuration_path(state); // Prefix with _ to silence warning
    let css = generate_css(&font_color, font_size, &background_color);
    
    apply_css(&css, state);
    
    log_info(state, "Building window...");
    let window = create_window(app);
    
    let clock_label = Arc::new(create_clock_label()); // Wrap in Arc
    let grid = create_grid(&clock_label);
    
    window.set_child(Some(&grid));

    // Start the timer for updating the clock label
    start_clock_update(clock_label.clone()); // Pass the cloned Arc

    log_info(state, "Window built successfully.");
    window
}

fn load_theme(config: &Config, state: &Arc<Mutex<AppState>>) -> (String, String, f32) {
    let background_color = config.theme.background_color.clone();
    log_info(state, &format!("Background color: {}", background_color));

    let font_color = config.theme.font_color.clone();
    log_info(state, &format!("Font color: {}", font_color));

    let font_size = config.theme.font_size as f32; // Ensure font_size is a float
    log_info(state, &format!("Font size: {}", font_size));

    (background_color, font_color, font_size)
}

fn load_configuration_path(state: &Arc<Mutex<AppState>>) -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/$USER"));
    let config_file = format!("{}/.config/hypr/hyprclock.conf", home_dir);
    let config_path = Path::new(&config_file);
    log_info(state, &format!("Configuration file path: {}", config_path.display()));
    config_path.to_path_buf() // Return PathBuf for further usage
}

fn generate_css(font_color: &str, font_size: f32, background_color: &str) -> String {
    format!(
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
        font_color, font_size, background_color
    )
}

fn apply_css(css: &str, state: &Arc<Mutex<AppState>>) {
    let provider = CssProvider::new();
    provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Use the passed-in state instead of trying to create a new AppState
    log_debug(state, &format!("Generated CSS:\n{}", css));
}

fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .css_classes(vec!["window".to_string()])
        .build()
}

fn create_clock_label() -> Label {
    Label::builder()
        .label(get_current_time())
        .css_classes(vec!["clock".to_string()])
        .build()
}

fn create_grid(clock_label: &Arc<Label>) -> Grid {
    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

    grid.attach(&**clock_label, 0, 1, 2, 1); // Dereference Arc to get the Label
    
    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true); 

    grid.set_halign(gtk::Align::Center); 
    grid.set_valign(gtk::Align::Center); 

    grid
}

fn start_clock_update(clock_label: Arc<Label>) {
    glib::timeout_add_seconds_local(1, move || {
        let current_time = get_current_time();
        clock_label.set_label(&current_time);
        Continue
    });
}

fn get_current_time() -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}
