use crate::resources::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub fn print_keyboard_event(
    mut state: ResMut<KeyboardState>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
) {
    for event in state.event_reader.iter(&keyboard_input_events) {
        if let Some(key_code) = event.key_code {
            println!("{0:?}: {1:?}", event.state, key_code);
        }
    }
}
