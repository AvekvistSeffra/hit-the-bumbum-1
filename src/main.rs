use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod ui_plugin;

use ui_plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UiPlugin)
        .run();
}
