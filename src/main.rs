mod components;
mod entities;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::resources::*;
use crate::systems::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::ClearColor;
use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_event::<GameEvent>()
        .insert_resource(WindowDescriptor {
            title: "Bevy Sokoban!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgba(0.95, 0.95, 0.95, 1.0)))
        .init_resource::<SoundHandles>()
        .init_resource::<Map>()
        .init_resource::<TileSize>()
        .init_resource::<Gameplay>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, print_keyboard_event)
        .add_system_to_stage(CoreStage::PreUpdate, input)
        .add_system_to_stage(CoreStage::Update, translation)
        .add_system_to_stage(CoreStage::Update, animate_sprite)
        .add_system_to_stage(CoreStage::Update, label_update)
        .add_system_to_stage(CoreStage::Update, event_listener)
        .add_system_to_stage(CoreStage::PostUpdate, gameplay_state)
        .run();
}
