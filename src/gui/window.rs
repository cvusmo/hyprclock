//src/gui/window.rs
//github.com/cvusmo/hyprclock

use gtk4 as gtk;
use glib::ControlFlow::Continue;
use gtk::{prelude::*, Application, ApplicationWindow, 
    Grid, Label, Switch, CssProvider, gdk::Display};
use std::path::Path;
use crate::Config;

pub fn build_ui(app: &Application, config: &Config) -> ApplicationWindow {
    
    // Load configuration styles
    let _background_color = config.theme.background_color.as_str();
    let _text_color = config.theme.text_color.as_str();
    let _font_size = config.theme.font_size;
    
    let clock_label = Label::builder()
        .label(get_current_time())
        .build();

    // Applies style
    let provider = CssProvider::new();
    provider.load_from_path(Path::new("hyprclock.conf"));
    // TODO: add LOGGER for error, debug, info

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Dark/Light mode switch 
    let switch = Switch::builder().build();

    // create 3x4 grid for window
    let grid = Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .build();

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

