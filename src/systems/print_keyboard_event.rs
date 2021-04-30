use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

// キーボードの入力を標準出力する
pub fn print_keyboard_event(
    mut state_event_reader: EventReader<KeyboardInput>
    // mut state: ResMut<KeyboardState>,
    // keyboard_input_events: Res<Events<KeyboardInput>>,
){
    for event in state_event_reader.iter(){
        if let Some(key_code) = event.key_code {
            println!("{0:?}: {1:?}", event.state, key_code); // {:?} は fmt::Debugの実装が使われる
        }
    }
    // for event in state.event_reader.iter(){
    //     if let Some(key_code) = event.key_code {
    //         println!("{0:?}: {1:?}", event.state, key_code); // {:?} は fmt::Debugの実装が使われる
    //     }
    // }
}