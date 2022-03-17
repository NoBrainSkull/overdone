use bevy::prelude::*;

mod load_config;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(load_config::LoadConfigPlugin)
        .add_plugin(map::TiledMapPlugin)
        .run();
}
