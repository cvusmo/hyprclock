use std::cell::Cell;
use std::rc::Rc;

use gio::Settings;

use glib::clone;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib, Align, Application, ApplicationWindow, Button, Orientation, Switch};

const APP_ID: &str = "org.cvusmo.Hyprclock";

fn main() -> glib::ExitCode {

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {

    let settings = Settings::new(APP_ID);

    let is_switch_enabled = settings.boolean("is-switch-enabled");

    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();
    
    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean("is-switch-enabled", is_enabled)
            .expect("Could not set setting.");
        
        glib::Propagation::Proceed
    });

    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }
    ));
    
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&switch);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyprclock")
        .child(&gtk_box)
        .build();

    window.present();
}
