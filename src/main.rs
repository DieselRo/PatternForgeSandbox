#![allow(dead_code)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use core::plugin::CorePlugin;
use player::plugin::PlayerPlugin;

mod states;
mod core;
mod player;
mod enemy;
mod ui;
mod editor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<states::AppState>()
        .add_plugin(CorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(enemy::plugin::EnemyPlugin)
        .add_plugin(ui::plugin::UiPlugin)
        // .add_plugin(editor::plugin::EditorPlugin)
        .add_plugins(WorldInspectorPlugin::new().run_if(cfg!(debug_assertions)))
        .run();
}
