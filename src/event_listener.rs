pub mod event_listener {
    use std::borrow::Borrow;
    use glib::Sender;
    use rdev::{Button, Event, EventType, listen as rdevListen};

    pub fn listen(sender: Sender<&str>) {
        let s: &Sender<&str> = sender.borrow();
        let event_handler = |event: Event| {
            match event.event_type {
                EventType::MouseMove {
                    x: _x,
                    y: _y,
                } => {}

                EventType::Wheel { delta_x: _delta_x, delta_y: _delta_y } => {}
                EventType::ButtonPress(button) => {
                    match button {
                        Button::Left => {
                            // This fails. I am stupid!
                            s.send("button left pressed").expect("Error");
                        }
                        Button::Right => {
                            println!("button right pressed");
                        }
                        Button::Middle => {
                            println!("button middle pressed")
                        }
                        Button::Unknown(_) => {
                            println!("button unkown pressed")
                        }
                    }
                }
                EventType::ButtonRelease(button) => {
                    match button {
                        Button::Left => {
                            println!("button left released")
                        }
                        Button::Right => {
                            println!("button right released")
                        }
                        Button::Middle => {
                            println!("button middle released")
                        }
                        Button::Unknown(_) => {
                            println!("button unkown released")
                        }
                    }
                }
                EventType::KeyPress(_) => {
                    // println!("key pressed")
                }
                EventType::KeyRelease(_) => {
                    // println!("key release")
                }
            }
        };

        if let Err(error) = rdevListen(event_handler) {
            println!("Error: {:?}", error)
        }
    }
}




