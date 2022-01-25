use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod hello_plugin;
mod ui_plugin;

use hello_plugin::HelloPlugin;
use ui_plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(HelloPlugin)
        .add_plugin(UiPlugin)
        .run();
}
