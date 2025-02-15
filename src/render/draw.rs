use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        query::ROQueryItem,
        system::{
            lifetimeless::{Read, SRes},
            SystemParamItem,
        },
    },
    log::error,
    render::{
        mesh::GpuBufferInfo,
        render_phase::{RenderCommand, RenderCommandResult, TrackedRenderPass},
        render_resource::PipelineCache,
        view::ViewUniformOffset,
    },
};

use super::{
    binding::{TilemapBindGroups, TilemapViewBindGroup},
    buffer::{DynamicOffsetComponent, TilemapUniform},
    chunk::RenderChunkStorage,
    extract::ExtractedTilemap,
};

pub type DrawTilemap = (
    SetPipeline,
    SetTilemapViewBindGroup<0>,
    SetTilemapUniformBufferBindGroup<1>,
    SetTilemapColorTextureBindGroup<2>,
    SetTilemapStorageBufferBindGroup<3>,
    DrawTileMesh,
);

pub type DrawTilemapPureColor = (
    SetPipeline,
    SetTilemapViewBindGroup<0>,
    SetTilemapUniformBufferBindGroup<1>,
    DrawTileMesh,
);

pub struct SetPipeline;
impl RenderCommand<Transparent2d> for SetPipeline {
    type Param = SRes<PipelineCache>;

    type ViewWorldQuery = ();

    type ItemWorldQuery = ();

    #[inline]
    fn render<'w>(
        item: &Transparent2d,
        _view: ROQueryItem<'w, Self::ViewWorldQuery>,
        _entity: ROQueryItem<'w, Self::ItemWorldQuery>,
        pipeline_cache: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let pipeline_cache = pipeline_cache.into_inner();
        if let Some(pipeline) = pipeline_cache.get_render_pipeline(item.pipeline) {
            pass.set_render_pipeline(pipeline);
            RenderCommandResult::Success
        } else {
            panic!(
                "Failed to get render pipeline!\n{:?}",
                pipeline_cache.get_render_pipeline_state(item.pipeline)
            );
        }
    }
}

pub struct SetTilemapViewBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetTilemapViewBindGroup<I> {
    type Param = ();

    type ViewWorldQuery = (Read<ViewUniformOffset>, Read<TilemapViewBindGroup>);

    type ItemWorldQuery = ();

    #[inline]
    fn render<'w>(
        _item: &Transparent2d,
        (view_uniform_offset, view_bind_group): ROQueryItem<'w, Self::ViewWorldQuery>,
        _entity: ROQueryItem<'w, Self::ItemWorldQuery>,
        _param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &view_bind_group.value, &[view_uniform_offset.offset]);

        RenderCommandResult::Success
    }
}

pub struct SetTilemapUniformBufferBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetTilemapUniformBufferBindGroup<I> {
    type Param = SRes<TilemapBindGroups>;

    type ViewWorldQuery = ();

    type ItemWorldQuery = Read<DynamicOffsetComponent<TilemapUniform>>;

    #[inline]
    fn render<'w>(
        _item: &Transparent2d,
        _view: ROQueryItem<'w, Self::ViewWorldQuery>,
        uniform_data: ROQueryItem<'w, Self::ItemWorldQuery>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(tilemap_uniform_bind_group) =
            bind_groups.into_inner().tilemap_uniform_buffer.as_ref()
        {
            pass.set_bind_group(I, tilemap_uniform_bind_group, &[uniform_data.index()]);
            RenderCommandResult::Success
        } else {
            error!("Failed to get tilemap uniform bind group!");
            RenderCommandResult::Failure
        }
    }
}

pub struct SetTilemapStorageBufferBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetTilemapStorageBufferBindGroup<I> {
    type Param = SRes<TilemapBindGroups>;

    type ViewWorldQuery = ();

    type ItemWorldQuery = Read<ExtractedTilemap>;

    #[inline]
    fn render<'w>(
        _item: &Transparent2d,
        _view: ROQueryItem<'w, Self::ViewWorldQuery>,
        tilemap: ROQueryItem<'w, Self::ItemWorldQuery>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(bind_group) = bind_groups
            .into_inner()
            .tilemap_storage_buffers
            .get(&tilemap.id)
        {
            pass.set_bind_group(I, bind_group, &[]);
            RenderCommandResult::Success
        } else {
            error!("Failed to get tilemap storage bind group!");
            RenderCommandResult::Failure
        }
    }
}

pub struct SetTilemapColorTextureBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetTilemapColorTextureBindGroup<I> {
    type Param = SRes<TilemapBindGroups>;

    type ViewWorldQuery = ();

    type ItemWorldQuery = Read<ExtractedTilemap>;

    #[inline]
    fn render<'w>(
        _item: &Transparent2d,
        _view: ROQueryItem<'w, Self::ViewWorldQuery>,
        tilemap: ROQueryItem<'w, Self::ItemWorldQuery>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(bind_group) = &bind_groups
            .into_inner()
            .colored_textures
            .get(tilemap.texture.as_ref().unwrap().handle())
        {
            pass.set_bind_group(I, bind_group, &[]);
            RenderCommandResult::Success
        } else {
            error!("Filed to get color texture bind group!");
            RenderCommandResult::Failure
        }
    }
}

pub struct DrawTileMesh;
impl RenderCommand<Transparent2d> for DrawTileMesh {
    type Param = SRes<RenderChunkStorage>;

    type ViewWorldQuery = ();

    type ItemWorldQuery = Read<ExtractedTilemap>;

    #[inline]
    fn render<'w>(
        _item: &Transparent2d,
        _view: ROQueryItem<'w, Self::ViewWorldQuery>,
        tilemap: ROQueryItem<'w, Self::ItemWorldQuery>,
        render_chunks: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(chunks) = render_chunks.into_inner().get_chunks(tilemap.id) {
            for chunk in chunks.values() {
                if !chunk.visible {
                    continue;
                }

                if let Some(gpu_mesh) = &chunk.gpu_mesh {
                    pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                    match &gpu_mesh.buffer_info {
                        GpuBufferInfo::Indexed {
                            buffer,
                            count,
                            index_format,
                        } => {
                            pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                            pass.draw_indexed(0..*count, 0, 0..1);
                        }
                        GpuBufferInfo::NonIndexed => {
                            pass.draw(0..gpu_mesh.vertex_count, 0..1);
                        }
                    }
                }
            }
        }

        RenderCommandResult::Success
    }
}
