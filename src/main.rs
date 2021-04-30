// モジュールを宣言する
mod components;
mod entities;
mod events;         // src/events.rs
mod resources;      // src/resources/*
mod systems;

use crate::events::*;
use crate::resources::*;
use crate::systems::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::pass::ClearColor
};

#[bevy_main] //ボイラープレートの定義(0.4から)
fn main() {
    App::build()
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
        .init_resource::<Map>() // Mapを読み込み
        .init_resource::<TileSize>()
        .init_resource::<Gameplay>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        // イベント設定
        .add_system_to_stage(CoreStage::PreUpdate, print_keyboard_event.system())
        .add_system_to_stage(CoreStage::PreUpdate, input.system())
        // 毎フレームのアップデート時
        .add_system_to_stage(CoreStage::Update, translation.system())
        .add_system_to_stage(CoreStage::Update, animate_sprite.system())
        .add_system_to_stage(CoreStage::Update, label_update.system())
        .add_system_to_stage(CoreStage::Update, event_listener.system())
        // アップデート後の更新
        .add_system_to_stage(CoreStage::PostUpdate, gameplay_state.system())
        .run();
}
