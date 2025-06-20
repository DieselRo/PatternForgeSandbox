use bevy::prelude::*;

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
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<states::AppState>()
        .add_plugins(CorePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(UiPlugin)
        // .add_plugins(EditorPlugin)
        .run();
}
