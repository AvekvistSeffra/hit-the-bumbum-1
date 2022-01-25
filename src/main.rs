use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod hello_plugin;
mod ui_plugin;

use hello_plugin::HelloPlugin;
use ui_plugin::UiPlugin;

struct SpriteSheets {
    map_tiles: Handle<TextureAtlas>,
}

fn use_sprites(
    handles: Res<SpriteSheets>,
    atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
) {
    if let Some(atlas) = atlases.get(&handles.map_tiles) {
        // do something with the texture atlas
    }

    if let Some(map_tex) = images.get("map.png") {
        // what is wrong with my linting or whatever
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(HelloPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(load_assets)
        .run();
}
