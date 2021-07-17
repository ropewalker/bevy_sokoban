use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub fn print_keyboard_event(mut keyboard_input_event_reader: EventReader<KeyboardInput>) {
    for event in keyboard_input_event_reader.iter() {
        if let Some(key_code) = event.key_code {
            println!("{0:?}: {1:?}", event.state, key_code);
        }
    }
}
