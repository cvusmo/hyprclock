// src/gui/window.rs
// github.com/cvusmo/hyprclock

use crate::configuration::config::Config;
use crate::configuration::general::GeneralConfig;
use crate::configuration::logger::{log_debug, log_info, AppState};
use glib::ControlFlow::Continue;
use gtk::{
    gdk::Display, pango::WrapMode, prelude::*, Application, ApplicationWindow, CssProvider, Grid,
    Justification, Label,
};
use gtk4 as gtk;
use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

// Function to Build UI
pub fn build_ui(
    app: &Application,
    config: &Config,
    state: &Arc<Mutex<AppState>>,
    debug_mode: bool,
) -> ApplicationWindow {
    // Build UI
    log_info(state, "Building UI...");
    let initial_width = 200;
    let initial_height = 150;
    let window = create_window(app, state, initial_width, initial_height);

    // Load config
    log_info(state, "Loading config...");
    let _config_path = load_configuration_path(state); // TODO: incorporate config_path

    // Load theme and apply CSS
    let (background_color, font_color, _font_size) = config.theme.load_theme(&state);
    let css = generate_css(
        &font_color,
        &background_color,
        initial_width,
        initial_height,
    );
    apply_css(&css, state);

    // Create clock
    let clock_label = create_clock_label(&config.general);

    // Set initial clock label size
    clock_label.set_width_request(initial_width);
    clock_label.set_height_request(initial_height);

    // Debug Mode enabled
    let debug_label = if debug_mode {
        Some(create_debug_label())
    } else {
        None
    };

    // Create grid
    let grid = create_grid(&clock_label, debug_label.as_ref());
    window.set_child(Some(&grid));

    // Start the timer for updating the clock label
    start_clock_update(clock_label.clone(), config.general.clone());

    // Dynamic Window Size Handling
    window.connect_notify_local(Some("default-width"), {
        let clock_label_clone = clock_label.clone();
        let font_color_clone = font_color.clone();
        let background_color_clone = background_color.clone();
        let state_clone = Arc::clone(state);

        move |window, _| {
            let width = window.default_width();
            let height = window.default_height();
            println!("Width updated: {}, Height: {}", width, height);
            if width > 10 && height > 10 {
                clock_label_clone.set_width_request(width);
                clock_label_clone.set_height_request(height);

                let updated_css =
                    generate_css(&font_color_clone, &background_color_clone, width, height);
                apply_css(&updated_css, &state_clone);
            }
        }
    });

    window.connect_notify_local(Some("default-height"), {
        let clock_label_clone = clock_label.clone();
        let font_color_clone = font_color.clone();
        let background_color_clone = background_color.clone();
        let state_clone = Arc::clone(state);

        move |window, _| {
            let width = window.default_width();
            let height = window.default_height();
            println!("Width updated: {}, Height: {}", width, height);
            if width > 10 && height > 10 {
                clock_label_clone.set_width_request(width);
                clock_label_clone.set_height_request(height);

                let updated_css =
                    generate_css(&font_color_clone, &background_color_clone, width, height);
                apply_css(&updated_css, &state_clone);
            }
        }
    });

    // Window built
    log_info(state, "Window built successfully.");
    window
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
        .css_classes(vec!["window".to_string()])
        .default_width(width)
        .default_height(height)
        .build()
}

// Function for clock label
fn create_clock_label(config: &GeneralConfig) -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label(&get_current_time(config))
            .justify(Justification::Fill)
            .wrap(true)
            .wrap_mode(WrapMode::WordChar)
            .max_width_chars(-1)
            .css_classes(vec!["clock".to_string()])
            .build(),
    )
}

// Function to Start Clock Update
fn start_clock_update(clock_label: Arc<Label>, general_config: GeneralConfig) {
    glib::timeout_add_seconds_local(1, move || {
        let current_time = get_current_time(&general_config);
        clock_label.set_label(&current_time);
        Continue
    });
}

// Function to get current time
fn get_current_time(config: &GeneralConfig) -> String {
    use chrono::{DateTime, Local};

    let now: DateTime<Local> = Local::now();

    match (config.clock_format.as_str(), config.time_precision.as_str()) {
        ("24-hour", "short") => now.format("%H:%M").to_string(),
        ("24-hour", "long") => now.format("%H:%M:%S").to_string(),
        ("12-hour", "short") => now.format("%I:%M %p").to_string(),
        ("12-hour", "long") => now.format("%I:%M:%S %p").to_string(),
        _ => now.format("%H:%M:%S").to_string(),
    }

    // now.format("%H:%M:%S").to_string()
    // now.format("%H:%M").to_string()
}

// Function to create debug label
fn create_debug_label() -> Arc<Label> {
    Arc::new(
        Label::builder()
            .label("Debug")
            .css_classes(vec!["debug-label".to_string()])
            .build(),
    )
}

// Function to create grid
fn create_grid(clock_label: &Arc<Label>, debug_label: Option<&Arc<Label>>) -> Grid {
    let grid = Grid::builder().row_spacing(10).column_spacing(10).build();

    // Attach clock
    grid.attach(clock_label.as_ref(), 0, 1, 2, 1); // Use `as_ref` to dereference Arc

    // Attach debug label if it exists
    if let Some(label) = debug_label {
        grid.attach(label.as_ref(), 0, 0, 2, 1); // Use `as_ref` to dereference Arc
        label.set_hexpand(true);
        label.set_vexpand(true);
    }

    clock_label.set_hexpand(true);
    clock_label.set_vexpand(true);

    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);

    grid
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

// Function to generate CSS
fn generate_css(font_color: &str, background_color: &str, width: i32, height: i32) -> String {
    let base_dimension = width.min(height);
    let font_size = (base_dimension as f32 * 0.1).max(12.0); // Minimum font size
    println!("Width updated: {}, Height: {}", width, height);

    format!(
        "
        .clock {{
            color: {};
            font-size: {}px;
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

// Function to apply CSS
fn apply_css(css: &str, state: &Arc<Mutex<AppState>>) {
    let provider = CssProvider::new();
    provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    log_debug(state, &format!("Generated CSS:\n{}", css));
}
