// src/gui/calendar.rs
use crate::configuration::config::Config;
use crate::configuration::logger::{log_error, log_info, AppState};
use crate::gui::clock::ClockModule;
use chrono::Datelike;
use glib::{DateTime, TimeZone};
use gtk4::{
    prelude::*, ApplicationWindow, Button, Calendar, Dialog, Entry, Label as GtkLabel, Popover,
};
use icalendar::{Calendar as ICalendar, Component, Event, EventLike};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CalendarModule {
    calendar: Calendar,
    popover: Popover,
    clock: Arc<ClockModule>,
}

impl CalendarModule {
    pub fn new(
        parent: &Button,
        _config: &Config,
        state: &Arc<Mutex<AppState>>,
        clock: Arc<ClockModule>,
    ) -> Self {
        let calendar = Calendar::new();
        let popover = Popover::builder()
            .child(&calendar)
            .autohide(true)
            .css_classes(vec!["calendar-popover".to_string()])
            .build();
        popover.set_parent(parent);

        log_info(state, "Calendar module initialized");

        let state_clone = Arc::clone(state);
        let parent_clone = parent.clone();
        let calendar_module = Self {
            calendar: calendar.clone(),
            popover,
            clock,
        };
        // Clone the module for use in the closure so that we do not move the value.
        let calendar_module_clone = calendar_module.clone();
        calendar.connect_day_selected(move |cal| {
            let date = cal.date();
            log_info(
                &state_clone,
                &format!(
                    "Selected date: {}-{}-{}",
                    date.year(),
                    date.month(),
                    date.day_of_month()
                ),
            );
            calendar_module_clone.show_schedule_dialog(&parent_clone, date, &state_clone);
        });

        calendar_module
    }

    pub fn show(&self) {
        self.popover.popup();
    }

    pub fn generate_tooltip(clock: &ClockModule) -> String {
        let now = clock.get_current_datetime();
        let year = now.year();
        let month = now.month() as u32;
        let day = now.day_of_month() as u32;
        format!(
            "<big>{} {}</big>\n<tt><small>{}</small></tt>",
            year,
            now.format("%B").expect("Format failed"),
            Self::generate_calendar(month, year, day)
        )
    }

    fn generate_calendar(month: u32, year: i32, highlight_day: u32) -> String {
        // Construct the first day as an ISO 8601 string and convert it.
        let first_day_str = format!("{:04}-{:02}-{:02}T00:00:00Z", year, month, 1);
        let first_day = DateTime::from_iso8601(&first_day_str, None).expect("Invalid date");
        // Determine next month (adjusting for December)
        let (next_year, next_month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let next_month_str = format!("{:04}-{:02}-{:02}T00:00:00Z", next_year, next_month, 1);
        let next_month = DateTime::from_iso8601(&next_month_str, None).expect("Invalid date");
        // Subtract one day (86400 seconds) from the start of next month by converting to a Unix timestamp.
        let last_day_unix = next_month.to_unix() - 86400; // 86400 seconds in a day
        let last_day = DateTime::from_unix_utc(last_day_unix).expect("Invalid date");
        let days_in_month = last_day.day_of_month() as u32;

        let mut calendar = String::new();
        calendar.push_str("Mo Tu We Th Fr Sa Su\n");

        // Get first day's weekday and adjust so week starts on Monday.
        let weekday = (first_day.day_of_week() as u32) % 7; // assume 1=Mon, 7=Sun
        let offset = (weekday + 6) % 7;
        for _ in 0..offset {
            calendar.push_str("   ");
        }
        let mut current_weekday = offset;
        for d in 1..=days_in_month {
            if d == highlight_day {
                calendar.push_str(&format!("[{:2}] ", d));
            } else {
                calendar.push_str(&format!("{:2}  ", d));
            }
            current_weekday += 1;
            if current_weekday == 7 {
                calendar.push_str("\n");
                current_weekday = 0;
            }
        }
        calendar
    }

    fn show_schedule_dialog(&self, parent: &Button, date: DateTime, state: &Arc<Mutex<AppState>>) {
        // Get the top-level window from the parent widget.
        let window = parent
            .ancestor(gtk4::ApplicationWindow::static_type())
            .and_then(|w| w.downcast::<ApplicationWindow>().ok());
        if let Some(window) = window {
            let events = Self::get_events_for_date(&date);
            let dialog = Dialog::with_buttons(
                Some(&format!(
                    "Schedule for {}-{}-{}",
                    date.year(),
                    date.month(),
                    date.day_of_month()
                )),
                Some(&window),
                gtk4::DialogFlags::MODAL,
                &[
                    ("Close", gtk4::ResponseType::Close),
                    ("Add Event", gtk4::ResponseType::Accept),
                ],
            );
            let content = dialog.content_area();
            let event_list = GtkLabel::new(Some(&events.join("\n")));
            content.append(&event_list);

            let state_clone = Arc::clone(state);
            let date_clone = date.clone();
            let calendar_clone = self.clone();
            dialog.connect_response(move |dlg, response| {
                if response == gtk4::ResponseType::Accept {
                    calendar_clone.add_event_form(&window, &date_clone, &state_clone);
                } else if response == gtk4::ResponseType::Close {
                    Self::launch_thunderbird_calendar(&date_clone);
                }
                dlg.close();
            });

            dialog.show();
        }
    }

    fn add_event_form(
        &self,
        parent: &ApplicationWindow,
        date: &DateTime,
        state: &Arc<Mutex<AppState>>,
    ) {
        let dialog = Dialog::with_buttons(
            Some("Add Event"),
            Some(parent),
            gtk4::DialogFlags::MODAL,
            &[
                ("Save", gtk4::ResponseType::Accept),
                ("Cancel", gtk4::ResponseType::Cancel),
            ],
        );
        let content = dialog.content_area();
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Event name"));
        content.append(&entry);

        let state_clone = Arc::clone(state);
        let date_clone = date.clone();
        let clock_clone = Arc::clone(&self.clock);
        dialog.connect_response(move |dlg, response| {
            if response == gtk4::ResponseType::Accept {
                let event_name = entry.text().to_string();
                if !event_name.is_empty() {
                    if let Err(e) =
                        Self::save_event(&date_clone, &event_name, &state_clone, &clock_clone)
                    {
                        log_error(
                            &state_clone,
                            &format!("Failed to save event '{}': {}", event_name, e),
                        );
                    } else {
                        log_info(
                            &state_clone,
                            &format!(
                                "Saved event '{}' for {}-{}-{}",
                                event_name,
                                date_clone.year(),
                                date_clone.month(),
                                date_clone.day_of_month()
                            ),
                        );
                    }
                }
            }
            dlg.close();
        });

        dialog.show();
    }

    fn get_events_for_date(date: &DateTime) -> Vec<String> {
        let ics_path = Self::get_ics_path();
        match fs::read_to_string(&ics_path) {
            Ok(contents) => match contents.parse::<ICalendar>() {
                Ok(calendar) => calendar
                    .components
                    .into_iter()
                    .filter_map(|component| {
                        component.as_event().and_then(|event| {
                            event.get_start().and_then(|dtstart| {
                                let dtstart_date = match dtstart {
                                    icalendar::DatePerhapsTime::Date(d) => {
                                        let iso_str = format!(
                                            "{:04}-{:02}-{:02}",
                                            d.year(),
                                            d.month(),
                                            d.day()
                                        );
                                        DateTime::from_iso8601(&iso_str, Some(&TimeZone::utc()))
                                            .expect("Invalid date")
                                    }
                                    icalendar::DatePerhapsTime::DateTime(
                                        icalendar::CalendarDateTime::Utc(dt),
                                    ) => {
                                        let iso_str = dt.to_rfc3339();
                                        DateTime::from_iso8601(&iso_str, None)
                                            .expect("Invalid datetime")
                                    }
                                    icalendar::DatePerhapsTime::DateTime(
                                        icalendar::CalendarDateTime::Floating(_),
                                    ) => {
                                        return None; // Skip floating times
                                    }
                                    icalendar::DatePerhapsTime::DateTime(
                                        icalendar::CalendarDateTime::WithTimezone { .. },
                                    ) => {
                                        return None; // Skip timezone-specific for now
                                    }
                                };
                                if dtstart_date.year() == date.year()
                                    && dtstart_date.month() == date.month()
                                    && dtstart_date.day_of_month() == date.day_of_month()
                                {
                                    Some(event.get_summary().unwrap_or("Unnamed event").to_string())
                                } else {
                                    None
                                }
                            })
                        })
                    })
                    .collect(),
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    }

    fn save_event(
        date: &DateTime,
        summary: &str,
        _state: &Arc<Mutex<AppState>>,
        clock: &Arc<ClockModule>,
    ) -> std::io::Result<()> {
        let ics_path = Self::get_ics_path();
        let calendar = fs::read_to_string(&ics_path)
            .and_then(|contents| {
                contents
                    .parse::<ICalendar>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
            .unwrap_or_else(|_| ICalendar::new());
        let mut calendar = calendar;

        // Use the clock’s UTC chrono time but change its year, month, day to match the given date.
        let start_chrono = clock
            .get_chrono_utc_datetime()
            .with_year(date.year())
            .and_then(|dt| dt.with_month(date.month() as u32))
            .and_then(|dt| dt.with_day(date.day_of_month() as u32))
            .expect("Failed to set chrono datetime");
        let end_chrono = start_chrono + chrono::Duration::days(1) - chrono::Duration::seconds(1);

        let start = icalendar::CalendarDateTime::Utc(start_chrono);
        let end = icalendar::CalendarDateTime::Utc(end_chrono);

        let event = Event::new().summary(summary).starts(start).ends(end).done();
        calendar.push(event);
        fs::write(&ics_path, calendar.to_string())?;
        Ok(())
    }

    fn launch_thunderbird_calendar(_date: &DateTime) {
        Command::new("thunderbird")
            .arg("-calendar")
            .spawn()
            .expect("Failed to launch Thunderbird");
    }

    fn get_ics_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/"))
            .join(".thunderbird/calendar.ics")
    }
}
