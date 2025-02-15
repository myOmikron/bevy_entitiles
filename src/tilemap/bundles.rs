use bevy::{
    ecs::bundle::Bundle,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};

use super::map::{
    TilePivot, TileRenderSize, TilemapAnimations, TilemapLayerOpacities, TilemapName,
    TilemapSlotSize, TilemapStorage, TilemapTexture, TilemapTransform, TilemapType,
};

/// The bundle of the tilemap with no actual tiles.
#[derive(Bundle, Default, Debug, Clone)]
pub struct DataTilemapBundle {
    /// The name of the tilemap. This can be used in saving the tilemap.
    pub name: TilemapName,
    /// The render size of tiles in pixels.
    pub tile_render_size: TileRenderSize,
    /// The size of each slot in pixels. This can be different from the render size.
    /// And you can create margins and paddings.
    pub slot_size: TilemapSlotSize,
    /// The type of the tilemap.
    pub ty: TilemapType,
    /// The pivot of the tiles.
    pub tile_pivot: TilePivot,
}

impl Into<TilemapBundle> for DataTilemapBundle {
    fn into(self) -> TilemapBundle {
        TilemapBundle {
            name: self.name,
            tile_render_size: self.tile_render_size,
            slot_size: self.slot_size,
            ty: self.ty,
            tile_pivot: self.tile_pivot,
            ..Default::default()
        }
    }
}

/// The bundle of the tilemap with a texture.
#[derive(Bundle, Default, Debug, Clone)]
pub struct TilemapBundle {
    /// The name of the tilemap. This can be used in saving the tilemap.
    pub name: TilemapName,
    /// The render size of tiles in pixels.
    pub tile_render_size: TileRenderSize,
    /// The size of each slot in pixels. This can be different from the render size.
    /// And you can create margins and paddings.
    pub slot_size: TilemapSlotSize,
    /// The type of the tilemap.
    pub ty: TilemapType,
    /// The pivot of the tiles.
    pub tile_pivot: TilePivot,
    /// The opacities of each **rendered** layer.
    ///
    /// Only the top 4 layers will be rendered.
    pub layer_opacities: TilemapLayerOpacities,
    /// The storage of the tilemap. The entities of each tiles are divided into chunks and stored in it.
    ///
    /// You need to spawn an empty tilemap and assign it to the storage.
    pub storage: TilemapStorage,
    /// The transform of the tilemap. It's not the same one as `Transform`.
    /// If you want to move or rotate the tilemap, you need to change this.
    /// Modify the `Transform` component will not work.
    pub tilemap_transform: TilemapTransform,
    /// The texture of the tilemap.
    pub texture: TilemapTexture,
    /// All the animation sequences of the tilemap.
    pub animations: TilemapAnimations,
    /// Just to make sure the child sprites are correctly rendered.
    pub visibility: Visibility,
    /// Just to make sure the child sprites are correctly rendered.
    pub inherited_visibility: InheritedVisibility,
    /// Just to make sure the child sprites are correctly rendered.
    pub view_visibility: ViewVisibility,
    /// Modify `TilemapTransform` instead of this one.
    pub transform: Transform,
    /// Just to make sure the child sprites are correctly rendered.
    pub global_transform: GlobalTransform,
}

impl Into<DataTilemapBundle> for TilemapBundle {
    fn into(self) -> DataTilemapBundle {
        DataTilemapBundle {
            name: self.name,
            tile_render_size: self.tile_render_size,
            slot_size: self.slot_size,
            ty: self.ty,
            tile_pivot: self.tile_pivot,
            ..Default::default()
        }
    }
}

impl Into<PureColorTilemapBundle> for TilemapBundle {
    fn into(self) -> PureColorTilemapBundle {
        PureColorTilemapBundle {
            name: self.name,
            tile_render_size: self.tile_render_size,
            slot_size: self.slot_size,
            ty: self.ty,
            tile_pivot: self.tile_pivot,
            layer_opacities: self.layer_opacities,
            storage: self.storage,
            tilemap_transform: self.tilemap_transform,
            visibility: self.visibility,
            inherited_visibility: self.inherited_visibility,
            view_visibility: self.view_visibility,
            transform: self.transform,
            global_transform: self.global_transform,
        }
    }
}

/// The bundle of the tilemap without a texture. This can be cheaper.
#[derive(Bundle, Default, Debug, Clone)]
pub struct PureColorTilemapBundle {
    /// The name of the tilemap. This can be used in saving the tilemap.
    pub name: TilemapName,
    /// The render size of tiles in pixels.
    pub tile_render_size: TileRenderSize,
    /// The size of each slot in pixels. This can be different from the render size.
    /// And you can create margins and paddings.
    pub slot_size: TilemapSlotSize,
    /// The type of the tilemap.
    pub ty: TilemapType,
    /// The pivot of the tiles.
    pub tile_pivot: TilePivot,
    /// The opacities of each **rendered** layer.
    ///
    /// Only the top 4 layers will be rendered.
    pub layer_opacities: TilemapLayerOpacities,
    /// The storage of the tilemap. The entities of each tiles are divided into chunks and stored in it.
    ///
    /// You need to spawn an empty tilemap and assign it to the storage.
    pub storage: TilemapStorage,
    /// The transform of the tilemap. It's not the same one as `Transform`.
    /// If you want to move or rotate the tilemap, you need to change this.
    /// Modify the `Transform` component will not work.
    pub tilemap_transform: TilemapTransform,
    /// Just to make sure the child sprites are correctly rendered.
    pub visibility: Visibility,
    /// Just to make sure the child sprites are correctly rendered.
    pub inherited_visibility: InheritedVisibility,
    /// Just to make sure the child sprites are correctly rendered.
    pub view_visibility: ViewVisibility,
    /// Modify `TilemapTransform` instead of this one.
    pub transform: Transform,
    /// Just to make sure the child sprites are correctly rendered.
    pub global_transform: GlobalTransform,
}
