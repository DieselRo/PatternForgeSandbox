use bevy::asset::AssetPlugin;
use bevy::prelude::*;

mod core;
mod editor;
mod enemy;
mod player;
mod states;
mod ui;

use core::plugin::CorePlugin;
use enemy::plugin::EnemyPlugin;
use player::plugin::PlayerPlugin;
use ui::plugin::UiPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes_override: Some(true),
        ..Default::default()
    }))
    .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .init_state::<states::AppState>()
    .add_plugins(CorePlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemyPlugin)
    .add_plugins(UiPlugin);
    // .add_plugins(EditorPlugin);

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
}
