use std::fmt::Display;

use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_ron::RonAssetPlugin;
use bevy_egui::EguiPlugin;
use serde::Deserialize;

mod ui_plugin;

use ui_plugin::UiPlugin;

#[derive(Deserialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
struct GameItemDescriptionAsset {
    damage: f32,
    durability: f32,
    min_level: u8,
}

struct GameItemDescriptionHandle(Handle<GameItemDescriptionAsset>);

impl Display for GameItemDescriptionAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "damage: {}, durability: {}, minimum level: {}",
            self.damage, self.durability, self.min_level
        )
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(RonAssetPlugin::<GameItemDescriptionAsset>::new(&["item"]))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let handles = server.load_folder("items");

    for handle in handles {
        commands.insert_resource(GameItemDescriptionHandle(handle));
    }
}
