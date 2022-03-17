use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod loader;
mod tiled_map;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<tiled_map::TiledMap>()
            .add_asset_loader(loader::TiledLoader)
            .add_system(loader::process_loaded_tile_maps)
            .add_plugin(TilemapPlugin)
            .add_startup_system(startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let handle = asset_server.load("simple.tmx");
    let map_entity = commands.spawn().id();

    commands
        .entity(map_entity)
        .insert_bundle(tiled_map::TiledMapBundle {
            tiled_map: handle,
            map: Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}
