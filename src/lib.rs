use bevy::{app::Update, prelude::Plugin};
use math::EntiTilesMathPlugin;
use render::{texture, EntiTilesRendererPlugin};
use tilemap::EntiTilesTilemapPlugin;

#[cfg(feature = "algorithm")]
pub mod algorithm;
#[cfg(feature = "debug")]
pub mod debug;
#[cfg(feature = "ldtk")]
pub mod ldtk;
pub mod math;
pub mod render;
#[cfg(feature = "serializing")]
pub mod serializing;
pub mod tilemap;

pub const MAX_LAYER_COUNT: usize = 4;
pub const DEFAULT_CHUNK_SIZE: u32 = 16;

pub mod prelude {
    #[cfg(feature = "algorithm")]
    pub use crate::algorithm::{
        pathfinding::{Path, PathFinder},
        wfc::WfcRunner,
    };
    #[cfg(feature = "ldtk")]
    pub use crate::ldtk::resources::{LdtkAssets, LdtkLevelManager};
    pub use crate::math::{aabb::Aabb2d, TileArea};
    #[cfg(feature = "serializing")]
    pub use crate::serializing::{
        chunk::{
            load::{ChunkLoadCache, ChunkLoadConfig},
            save::{ChunkSaveCache, ChunkSaveConfig},
        },
        map::{load::TilemapLoader, save::TilemapSaver},
    };
    #[cfg(feature = "physics")]
    pub use crate::tilemap::physics::TileCollision;
    pub use crate::tilemap::{
        bundles::{PureColorTilemapBundle, TilemapBundle},
        chunking::camera::{CameraChunkUpdater, CameraChunkUpdation},
        map::{
            TilePivot, TileRenderSize, TilemapAnimations, TilemapLayerOpacities, TilemapName,
            TilemapSlotSize, TilemapStorage, TilemapTexture, TilemapTextureDescriptor,
            TilemapTransform, TilemapType,
        },
        tile::{TileAnimation, TileBuilder, TileLayer, TileUpdater},
    };
}

pub struct EntiTilesPlugin;

impl Plugin for EntiTilesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, texture::set_texture_usage);

        app.add_plugins((
            EntiTilesTilemapPlugin,
            EntiTilesRendererPlugin,
            EntiTilesMathPlugin,
            #[cfg(feature = "debug")]
            debug::EntiTilesDebugPlugin,
            #[cfg(feature = "algorithm")]
            algorithm::EntiTilesAlgorithmPlugin,
            #[cfg(feature = "serializing")]
            serializing::EntiTilesSerializingPlugin,
            #[cfg(feature = "ldtk")]
            ldtk::EntiTilesLdtkPlugin,
        ));
    }
}
