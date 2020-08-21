mod components;
mod entities;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::resources::*;
use crate::systems::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
        .add_event::<GameEvent>()
        .add_resource(WindowDescriptor {
            title: "Bevy Sokoban!".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgba(0.95, 0.95, 0.95, 1.0)))
        .init_resource::<KeyboardState>()
        .init_resource::<EventListenerState>()
        .init_resource::<Map>()
        .init_resource::<TileSize>()
        .init_resource::<Gameplay>()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::EVENT_UPDATE, print_keyboard_event.system())
        .add_system_to_stage(stage::EVENT_UPDATE, input_system.system())
        .add_system_to_stage(stage::PRE_UPDATE, movement_system.system())
        .add_system_to_stage(stage::UPDATE, translation_system.system())
        .add_system_to_stage(stage::UPDATE, animate_sprite_system.system())
        .add_system_to_stage(stage::UPDATE, label_update_system.system())
        .add_system_to_stage(stage::UPDATE, event_listener_system.system())
        .add_system_to_stage(stage::POST_UPDATE, gameplay_state_system.system())
        .run();
}
