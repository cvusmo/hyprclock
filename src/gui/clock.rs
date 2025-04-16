// src/gui/clock.rs
use crate::configuration::config::Config;
use crate::configuration::logger::{log_info, AppState};
use chrono::{DateTime as ChronoDateTime, Local, Utc};
use glib::DateTime;
use gtk4::Label;
use std::sync::{Arc, Mutex};

pub struct ClockModule {
    label: Arc<Label>,
}

impl ClockModule {
    pub fn new(config: &Config, state: &Arc<Mutex<AppState>>) -> Self {
        let label = Arc::new(
            Label::builder()
                .label(&config.general.get_current_time())
                .justify(gtk4::Justification::Center)
                .wrap(true)
                .wrap_mode(gtk4::pango::WrapMode::WordChar)
                .max_width_chars(-1)
                .css_classes(vec!["clock".to_string()])
                .build(),
        );

        let config_clone = config.clone();
        let label_clone = Arc::clone(&label);
        let state_clone = Arc::clone(state);
        glib::timeout_add_seconds_local(1, move || {
            let current_time = config_clone.general.get_current_time();
            label_clone.set_label(&current_time);
            log_info(
                &state_clone,
                &format!("Updated clock label to: {}", current_time),
            );
            glib::ControlFlow::Continue
        });

        log_info(state, "Clock module initialized");

        Self { label }
    }

    pub fn get_label(&self) -> &Arc<Label> {
        &self.label
    }

    pub fn get_time(&self, config: &Config) -> String {
        config.general.get_current_time()
    }

    pub fn get_current_datetime(&self) -> DateTime {
        let chrono_time = Local::now();
        let iso_str = chrono_time.to_rfc3339();
        DateTime::from_iso8601(&iso_str, None).expect("Failed to convert chrono to glib datetime")
    }

    pub fn get_chrono_utc_datetime(&self) -> ChronoDateTime<Utc> {
        Local::now().with_timezone(&Utc)
    }
}
