pub mod event_listener {
    use rdev::{Button, Event, EventType, listen as rdevListen};

    pub fn listen() {
        if let Err(error) = rdevListen(callback) {
            println!("Error: {:?}", error)
        }
    }

    fn callback(event: Event) {
        match event.event_type {
            EventType::MouseMove {
                x: _x,
                y: _y,
            } => {}

            EventType::Wheel { delta_x: _delta_x, delta_y: _delta_y } => {}
            EventType::ButtonPress(button) => {
                match button {
                    Button::Left => {
                        println!("button left pressed")
                    }
                    Button::Right => {
                        println!("button right pressed")
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
    }
}




