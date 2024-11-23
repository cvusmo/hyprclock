// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::{
    configuration::logger::{log_debug, log_info, AppState},
    Config,
};
use glib::ControlFlow::Continue;
use gtk::{gdk::Display, prelude::*, Application, ApplicationWindow, CssProvider, Grid, Label};
use gtk4 as gtk;
use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub fn build_ui(
    app: &Application,
    config: &Config,
    state: &Arc<Mutex<AppState>>,
    debug_mode: bool,
) -> ApplicationWindow {
    log_info(state, "Loading config...");

    // Load theme and aply CSS
    let (background_color, font_color, font_size) = load_theme(config, state);
    let _config_path = load_configuration_path(state); // TODO: incorporate config_path
    let css = generate_css(&font_color, font_size, &background_color);
    apply_css(&css, state);

    // Create window
    log_info(state, "Building window...");
    let window = create_window(app);

    // Create clock
    let clock_label = Arc::new(create_clock_label());

    // Debug Mode enabled
    let debug_label = if debug_mode {
        Some(Arc::new(create_debug_label()))
    } else {
        None
    };

    // Create grid
    let grid = create_grid(&clock_label, debug_label.as_ref());

    window.set_child(Some(&grid));

    // Start the timer for updating the clock label
    start_clock_update(clock_label.clone());

    // Resize window
    let base_font_size = font_size;
    window.size_allocate(window, &rectangle, base_font_size) {
    dynamic_font(&window, &clock_label, base_font_size);
    }

    log_info(state, "Window built successfully.");
    window
}

// Function to create window
fn create_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .resizable(true)
        .css_classes(vec!["window".to_string()])
        .build()
}

// Function for clock label
fn create_clock_label() -> Label {
    Label::builder()
        .label(get_current_time())
        .max_width_chars(-1)
        .css_classes(vec!["clock".to_string()])
        .build()
}

fn create_debug_label() -> Label {
    Label::builder()
        .label("Debug")
        .css_classes(vec!["debug-label".to_string()])
        .build()
}

// Function to create grid
fn create_grid(clock_label: &Arc<Label>, debug_label: Option<&Arc<Label>>) -> Grid {
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    // Attach clock
    grid.attach(&**clock_label, 0, 1, 2, 1); // Dereference Arc to get the Label

    // Attach debug label
    if let Some(label) = debug_label {
        grid.attach(&**label, 0, 0, 2, 1);
        label.set_hexpand(true);
        label.set_vexpand(true);
    }

    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);

    grid
}

// Function to Start Clock Update
fn start_clock_update(clock_label: Arc<Label>) {
    glib::timeout_add_seconds_local(1, move || {
        let current_time = get_current_time();
        clock_label.set_label(&current_time);
        Continue
    });
}

// Function to get current time
fn get_current_time() -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}

// Function to load theme
fn load_theme(config: &Config, state: &Arc<Mutex<AppState>>) -> (String, String, f32) {
    let background_color = config.theme.background_color.clone();
    log_info(state, &format!("Background color: {}", background_color));

    let font_color = config.theme.font_color.clone();
    log_info(state, &format!("Font color: {}", font_color));

    let font_size = config.theme.font_size as f32; // Ensure font_size is a float
    log_info(state, &format!("Font size: {}", font_size));

    (background_color, font_color, font_size)
}

// Function to load config path
fn load_configuration_path(state: &Arc<Mutex<AppState>>) -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/home/$USER"));
    let config_file = format!("{}/.config/hypr/hyprclock.conf", home_dir);
    let config_path = Path::new(&config_file);
    log_info(
        state,
        &format!("Configuration file path: {}", config_path.display()),
    );
    config_path.to_path_buf() // Return PathBuf for further usage
}

// Function to generate css
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
        .debug-label {{
            color: red;
            font-weight: bold;
        }}
        ",
        font_color, font_size, background_color
    )
}

// Function to apply css
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

// Function to update font size dynamically
fn dynamic_font(window: &ApplicationWindow, label: &Label, base_size: f32) {
    let allocation = window.allocation();
    let width = allocation.width() as f32;
    let height = allocation.height() as f32;

    // Calculate dynamic size based off window
    let new_font_size = base_size.min(width / 15.0).min(height / 15.0);

    // Apply updated font size via CSS
    let css = format! (
        "
        .clock {{
            font-size: {}px;
            text-align: center;
        }}
        ",
        new_font_size
    );
    let provider = CssProvider::new();
    provider.load_from_data(css.as_bytes());
    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
