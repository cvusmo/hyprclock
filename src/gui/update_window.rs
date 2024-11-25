// src/gui/update_window.rs
// github.com/cvusmo/hyprclock

use gtk::{gdk::Display, prelude::*, ApplicationWindow, Label};
use gtk4 as gtk;
use std::sync::Arc;

// Function to monitor window resizing events
pub fn monitor_css(window: &ApplicationWindow, clock_label: Arc<Label>) {
    // Clone necessary variables
    let clock_label_clone = clock_label.clone();

    // Connect to window resizing events
    window.connect_map(move |window| {
        handle_window_resize(window, &clock_label_clone);
    });
}

// Function to handle window resizing
fn handle_window_resize(window: &ApplicationWindow, clock_label: &Label) {
    // Get GDK surface associated with ApplicationWindow
    if let Some(surface) = window.surface() {
        let display = Display::default().expect("Unable to get default display");

        // Get monitor associated with the surface
        if let Some(monitor) = display.monitor_at_surface(&surface) {
            let monitor_width = monitor.geometry().width();
            let monitor_height = monitor.geometry().height();

            // Set width and height of the window
            let width = window.default_width();
            let height = window.default_height();

            println!(
                "Monitor: Width: {}, Height: {}, Window Resized - Width: {}, Height: {}",
                monitor_width, monitor_height, width, height
            );

            if width > 2 && height > 2 {
                // Adjust clock label size based on new window dimensions
                let new_font_size = calculate_dynamic_font_size(width, height);
                clock_label.set_css_classes(&[&format!("font-size: {}px;", new_font_size)]);
                println!("Adjusted clock label font size to: {}px", new_font_size);
            }
        } else {
            println!("Unable to determine monitor for the given surface.");
        }
    } else {
        println!("Unable to get surface from window.");
    }
}

// Function to calculate dynamic font size based on window dimensions
fn calculate_dynamic_font_size(width: i32, height: i32) -> f32 {
    // Use a more sophisticated calculation based on window area or other factors
    let area = width as f32 * height as f32;
    (area.sqrt() / 50.0).max(12.0) // Setting a reasonable minimum font size
}
