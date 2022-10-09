use std::borrow::Borrow;
use std::sync::mpsc;

use glib::PRIORITY_DEFAULT;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation};
use gtk::glib::MainContext;
use gtk::glib::Receiver;
use gtk::prelude::*;

use crate::event_listener::event_listener::listen;

pub mod event_listener;

const APP_ID: &str = "org.gtk_rs.mouseTracker";

fn main() {
    let app = UiApplication {
        app: Application::builder().application_id(APP_ID).build()
    };

    app.run();
}

struct UiApplication {
    app: Application,
}

impl UiApplication {
    fn run(&self) {
        &self.app.connect_activate(move |app: &Application| {
            let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

            let main_context = MainContext::default();

            main_context.spawn(async move {
                sender.send("hello").unwrap();
                listen(sender);
            });
            let label_value = "Number of times you've used the mouse";

            let label = Label::builder()
                .label(label_value)
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            let gtk_box = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

            gtk_box.append(&label);

            receiver.attach(
                None,
                move |value| {
                    label.set_label(value);
                    Continue(true)
                }
            );

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Track mouse")
                .child(&gtk_box)
                .build();

            window.present();
        });
        &self.app.run();
    }
}
