// src/gui/window.rs
use crate::configuration::config::Config;
use crate::configuration::logger::*;
use crate::gui::calendar::CalendarModule;
use crate::gui::clock::ClockModule;
use crate::gui::update_window::monitor_css;
use gtk4::{prelude::*, Application, ApplicationWindow, Button, Grid, Label};
use std::sync::{Arc, Mutex};

pub fn build_ui(
    app: &Application,
    config: &Config,
    state: &Arc<Mutex<AppState>>,
    debug_mode: bool,
) -> ApplicationWindow {
    log_info(state, "Building UI...");

    let initial_width = 400;
    let initial_height = 200;
    let window = create_window(app, state, initial_width, initial_height);

    let clock_module = Arc::new(ClockModule::new(config, state));
    let clock_label = clock_module.get_label();

    let clock_button = Button::builder()
        .child(&**clock_label)
        .css_classes(vec!["clock-button".to_string()])
        .build();

    let calendar_module =
        CalendarModule::new(&clock_button, config, state, Arc::clone(&clock_module));

    let debug_label = if debug_mode {
        Some(create_debug_label())
    } else {
        None
    };

    let grid = create_grid(&clock_button, debug_label.as_ref());
    window.set_child(Some(&grid));

    monitor_css(&window, Arc::clone(clock_label));

    let gesture = gtk4::GestureClick::new();
    let calendar_clone = calendar_module;
    gesture.connect_pressed(move |_, _, _, _| {
        calendar_clone.show();
    });
    clock_button.add_controller(gesture);

    log_info(state, "Window built successfully.");
    window
}

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
        .css_classes(vec!["window".to_string()])
        .default_width(width)
        .default_height(height)
        .build()
}

fn create_debug_label() -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label("Debug")
            .css_classes(vec!["debug-label".to_string()])
            .build(),
    )
}

fn create_grid(clock_button: &Button, debug_label: Option<&Arc<Label>>) -> Grid {
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    grid.attach(clock_button, 0, 1, 2, 1);

    if let Some(label) = debug_label {
        grid.attach(label.as_ref(), 0, 0, 2, 1);
        label.set_hexpand(true);
        label.set_vexpand(true);
    }

    clock_button.set_hexpand(true);
    clock_button.set_vexpand(true);

    grid.set_halign(gtk4::Align::Center);
    grid.set_valign(gtk4::Align::Center);

    grid
}
