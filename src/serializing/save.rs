use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query},
    },
    utils::HashMap,
};
use serde::Serialize;

use crate::tilemap::{map::Tilemap, tile::Tile};

use super::{SerializedTile, SerializedTilemap, TilemapLayer, PATH_TILES, TILEMAP_META, TILES};

pub struct TilemapSaverBuilder {
    path: String,
    map_name: String,
    texture_path: Option<String>,
    layers: u32,
    remove_map_after_done: bool,
}

impl TilemapSaverBuilder {
    pub fn new(path: String, map_name: String) -> Self {
        TilemapSaverBuilder {
            path,
            map_name,
            texture_path: None,
            layers: 0,
            remove_map_after_done: false,
        }
    }

    /// Set which layers to save. By default, only the texture layer is saved.
    /// If there's async algorithms performing when saving, you should save them.
    pub fn with_layer(mut self, layer: TilemapLayer) -> Self {
        self.layers |= layer as u32;
        self
    }

    /// Set the texture path to save.
    pub fn with_texture(mut self, texture_path: String) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    /// Despawn the tilemap after saving.
    pub fn remove_map_after_done(mut self) -> Self {
        self.remove_map_after_done = true;
        self
    }

    pub fn build(self, commands: &mut Commands, target: Entity) {
        commands.entity(target).insert(TilemapSaver {
            path: self.path,
            map_name: self.map_name,
            texture_path: self.texture_path,
            layers: self.layers,
            remove_map_after_done: self.remove_map_after_done,
        });
    }
}

#[derive(Component)]
pub struct TilemapSaver {
    pub(crate) path: String,
    pub(crate) map_name: String,
    pub(crate) texture_path: Option<String>,
    pub(crate) layers: u32,
    pub(crate) remove_map_after_done: bool,
}

pub fn save(
    mut commands: Commands,
    tilemaps_query: Query<(Entity, &Tilemap, &TilemapSaver)>,
    tiles_query: Query<&Tile>,
    #[cfg(feature = "algorithm")] path_tilemaps_query: Query<
        &crate::tilemap::algorithm::path::PathTilemap,
    >,
) {
    for (entity, tilemap, saver) in tilemaps_query.iter() {
        let map_path = saver.path.clone() + &saver.map_name + "\\";

        let serialized_tilemap = SerializedTilemap::from_tilemap(tilemap, saver);
        save_object(&map_path, TILEMAP_META, &serialized_tilemap);

        // texture
        if saver.layers & 1 != 0 {
            let serialized_tiles: Vec<Option<SerializedTile>> = tilemap
                .tiles
                .iter()
                .map(|e| {
                    if let Some(tile) = e {
                        Some(tiles_query.get(tile.clone()).unwrap().clone().into())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            save_object(&map_path, TILES, &serialized_tiles);
        }

        // algorithm
        #[cfg(feature = "algorithm")]
        if saver.layers & (1 << 1) != 0 {
            if let Ok(path_tilemap) = path_tilemaps_query.get(entity) {
                let serialized_path_map = super::SerializedPathTilemap {
                    tiles: path_tilemap
                        .tiles
                        .iter()
                        .map(|(index, tile)| {
                            (*index, super::SerializedPathTile { cost: tile.cost })
                        })
                        .collect::<HashMap<_, _>>(),
                };

                save_object(&map_path, PATH_TILES, &serialized_path_map);
            }
        }

        if saver.remove_map_after_done {
            commands.entity(entity).despawn();
        }

        commands.entity(entity).remove::<TilemapSaver>();
    }
}

pub fn save_object<T: Serialize>(path: &str, file_name: &str, object: &T) {
    let _ = create_dir_all(path);
    let _ = File::create(path.to_owned() + file_name)
        .unwrap_or(File::open(path.to_owned() + file_name).unwrap())
        .write(ron::to_string(object).unwrap().as_bytes());
}
