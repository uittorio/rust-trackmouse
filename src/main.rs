use std::cell::Cell;
use std::rc::Rc;

use gtk::{Application, ApplicationWindow, Button, Label, Orientation};
use gtk::glib::{clone, MainContext};
use gtk::prelude::*;

use crate::event_listener::event_listener::listen;

pub mod event_listener;

const APP_ID: &str = "org.gtk_rs.mouseTracker";

fn main() {
    let main_context = MainContext::default();

    main_context.spawn(async {
        listen();
    });

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label_value = "hello";
    let label = Label::builder()
        .label(label_value)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    label.set_label("Oh no, something else");

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

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&label);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();


    window.present();
}
