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

#[bevy_main]
fn main() {
    App::new()
        .add_event::<GameEvent>()
        .insert_resource(WindowDescriptor {
            title: "Bevy Sokoban!".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgba(0.95, 0.95, 0.95, 1.0)))
        .init_resource::<SoundHandles>()
        .init_resource::<Map>()
        .init_resource::<TileSize>()
        .init_resource::<Gameplay>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::PreUpdate, print_keyboard_event.system())
        .add_system_to_stage(CoreStage::PreUpdate, input.system())
        .add_system_to_stage(CoreStage::Update, translation.system())
        .add_system_to_stage(CoreStage::Update, animate_sprite.system())
        .add_system_to_stage(CoreStage::Update, label_update.system())
        .add_system_to_stage(CoreStage::Update, event_listener.system())
        .add_system_to_stage(CoreStage::PostUpdate, gameplay_state.system())
        .run();
}
