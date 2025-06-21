use bevy::prelude::*;
use bevy::asset::AssetPlugin;

mod states;
mod core;
mod player;
mod enemy;
mod ui;
mod editor;

use core::plugin::CorePlugin;
use player::plugin::PlayerPlugin;
use enemy::plugin::EnemyPlugin;
use ui::plugin::UiPlugin;
use editor::plugin::EditorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        .init_state::<states::AppState>()
        .add_plugins(CorePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(UiPlugin);
        // .add_plugins(EditorPlugin);

    #[cfg(debug_assertions)]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        app.add_plugins(WorldInspectorPlugin::new())
           .add_plugins(FrameTimeDiagnosticsPlugin::default())
           .add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}
