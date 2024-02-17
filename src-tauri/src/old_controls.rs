use core::time;
use std::{thread::sleep, time::{Duration, SystemTime}};

use gilrs::{Button, Event, Axis, Gilrs};
use enigo::*;

fn main()
{
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let stick_deadzone = 30.0 as f32;

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {

            active_gamepad = Some(id);
            match event {
                gilrs::EventType::AxisChanged(Axis::LeftStickX, mut value, _) => {
                    value = value * 100.0 as f32;
                    if value > stick_deadzone {
                        enigo.mouse_move_relative(1, 0);
                    } else if value < -stick_deadzone {
                        enigo.mouse_move_relative(-1, 0);
                    }
                },
                gilrs::EventType::AxisChanged(Axis::LeftStickY, mut value, _) => {
                    value = value * 100.0 as f32;
                    if value > stick_deadzone {
                        enigo.mouse_move_relative(0, -1);
                    } else if value < -stick_deadzone {
                        enigo.mouse_move_relative(0, 1);
                    }
                },
                gilrs::EventType::AxisChanged(Axis::RightStickX, mut value, _) => {
                    value = value * 100.0 as f32;
                    if value > stick_deadzone {
                        enigo.mouse_scroll_x(1);
                    } else if value < -stick_deadzone {
                        enigo.mouse_scroll_x(-1);
                    }
                },
                gilrs::EventType::AxisChanged(Axis::RightStickY, mut value, _) => {
                    value = value * 100.0 as f32;
                    if value > stick_deadzone {
                        enigo.mouse_scroll_y(-1);
                    } else if value < -stick_deadzone {
                        enigo.mouse_scroll_y(1);
                    }
                },
                _ => (),
            }
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            
            // let current_mouse_position = mouse::

            if gamepad.is_pressed(Button::South) {
                enigo.mouse_click(MouseButton::Left);
            }
            if gamepad.is_pressed(Button::East) {
                enigo.mouse_click(MouseButton::Right);
            }
            if gamepad.is_pressed(Button::Mode) {
                enigo.key_click(Key::Meta)
            }
        }
        sleep(Duration::from_millis(50));
    }
}