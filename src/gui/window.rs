// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::configuration::config::Config;
use crate::configuration::general::GeneralConfig;
use crate::configuration::logger::*;
use crate::gui::update_window::monitor_css;
use gtk::{prelude::*, Application, ApplicationWindow, Grid, Justification, Label};
use gtk4 as gtk;
use std::sync::{Arc, Mutex};

// Function to Build UI
pub fn build_ui(
    app: &Application,
    config: &Config, // Config is already validated and loaded (either from default or file)
    state: &Arc<Mutex<AppState>>,
    debug_mode: bool,
) -> ApplicationWindow {
    // Build UI
    log_info(state, "Building UI...");

    // Get initial width and height
    let initial_width = 400;
    let initial_height = 200;

    // Create application window
    let window = create_window(app, state, initial_width, initial_height);

    // Create clock label using validated GeneralConfig
    let clock_label = create_clock_label(&config.general);

    // Debug Mode enabled label
    let debug_label = if debug_mode {
        Some(create_debug_label())
    } else {
        None
    };

    // Create grid and set it as window child
    let grid = create_grid(&clock_label, debug_label.as_ref());
    window.set_child(Some(&grid));

    // Start the timer for updating the clock label using GeneralConfig
    config
        .general
        .clone()
        .start_clock_update(clock_label.clone(), state.clone());

    // Monitor window resizing events to adjust `clock_label`
    monitor_css(&window, clock_label);

    // Window built successfully
    log_info(state, "Window built successfully.");
    window
}

// Function to create the clock label
fn create_clock_label(config: &GeneralConfig) -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label(&config.get_current_time())
            .justify(Justification::Center)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .max_width_chars(-1)
            .css_classes(vec!["clock".to_string()]) // Uses CSS class for styling
            .build(),
    )
}

// Function to create window
fn create_window(
    app: &Application,
    state: &Arc<Mutex<AppState>>,
    width: i32,
    height: i32,
) -> ApplicationWindow {
    log_info(state, "Creating application window...");
    ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .resizable(true)
        .css_classes(vec!["window".to_string()]) // Uses CSS class for styling
        .default_width(width)
        .default_height(height)
        .build()
}

// Function to create the debug label
fn create_debug_label() -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label("Debug")
            .css_classes(vec!["debug-label".to_string()])
            .build(),
    )
}

// Function to create the grid
fn create_grid(clock_label: &Arc<Label>, debug_label: Option<&Arc<Label>>) -> Grid {
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    // Attach clock label
    grid.attach(clock_label.as_ref(), 0, 1, 2, 1);

    // Attach debug label if it exists
    if let Some(label) = debug_label {
        grid.attach(label.as_ref(), 0, 0, 2, 1);
        label.set_hexpand(true);
        label.set_vexpand(true);
    }

    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);

    grid
}
