#![allow(dead_code)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use core::plugin::CorePlugin;
use player::plugin::PlayerPlugin;

mod core;
mod editor;
mod enemy;
mod player;
mod states;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_state::<states::AppState>()
        .add_plugins(CorePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(enemy::plugin::EnemyPlugin)
        .add_plugins(ui::plugin::UiPlugin); // .add_plugins(editor::plugin::EditorPlugin)

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
