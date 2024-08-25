            use eframe::{egui, App};
use chrono::Local;
use std::fs;
use std::collections::HashMap;

fn main() {
    // Load settings from settings.ini
    let settings = load_settings("settings.ini");

    // Extract settings from the loaded configuration
    let title = settings.get("title").unwrap_or(&"Hyprclock".to_string()).clone();
    let width = settings.get("width").unwrap_or(&"400".to_string()).parse::<f32>().unwrap_or(400.0);
    let height = settings.get("height").unwrap_or(&"300".to_string()).parse::<f32>().unwrap_or(300.0);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(width, height)),
        ..Default::default()
    };

    eframe::run_native(
        &title,
        options,
        Box::new(|_cc| -> Result<Box<(dyn App + 'static)>, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
            Ok(Box::new(Hyprclock))
        }),
    ).unwrap();
}

struct Hyprclock;

impl Hyprclock {
    fn new() -> Self {
        Self
    }
}

impl eframe::App for Hyprclock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let time = Local::now().format("%H:%M:%S").to_string();
            ui.label(format!("Current time: {}", time));
        });
        ctx.request_repaint();
    }
}

// Function to load settings from an INI file
fn load_settings(file_path: &str) -> HashMap<String, String> {
    let mut settings = HashMap::new();
    
    if let Ok(contents) = fs::read_to_string(file_path) {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                settings.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    
    settings
}
}
        }
    }
    
    settings
}
