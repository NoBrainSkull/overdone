use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]

pub struct TiledMap {
    pub map: tiled::Map,
    pub tilesets: HashMap<u32, Handle<Image>>,
}

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub map: Map,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
