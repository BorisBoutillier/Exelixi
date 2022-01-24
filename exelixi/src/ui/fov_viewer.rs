use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::render_resource::{
    BindGroupEntry, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
};
use bevy::sprite::Material2dPipeline;
use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
            BufferInitDescriptor, BufferUsages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::prelude::*;

/// a a FOV viewer child whenver a Selected component is added to an entity with an Eye
pub fn spawn_fov_viewer_on_selected(
    mut commands: Commands,
    parents: Query<(Entity, &Eye), Added<Selected>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FovViewerMaterial>>,
) {
    for (parent, eye) in parents.iter() {
        let fov_viewer = commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: Vec3::new(eye.fov_range * 2.0, eye.fov_range * 2.0, 1.0),
                    ..Default::default()
                },
                material: materials.add(FovViewerMaterial {
                    fov_angle: eye.fov_angle,
                    n_sectors: eye.n_cells as u32,
                    color: Color::BLUE,
                    sector_alpha: 0.1,
                }),
                ..Default::default()
            })
            .id();
        commands.entity(parent).add_child(fov_viewer);
    }
}
pub fn despawn_fov_viewer_on_deselected(
    mut commands: Commands,
    fov_viewers: Query<(Entity, &Parent), With<Handle<FovViewerMaterial>>>,
    deselected: RemovedComponents<Selected>,
) {
    for entity in deselected.iter() {
        for (child, parent) in fov_viewers.iter() {
            if parent.0 == entity {
                commands.entity(child).despawn();
            }
        }
    }
}

#[derive(Component, Debug, Clone, TypeUuid)]
#[uuid = "516c3ab4-6a1c-4e7d-9795-6161ca083a1d"]
pub struct FovViewerMaterial {
    // Total angle of the FOV to show, will be show from -fov_angle/2 to fov_anglel2
    fov_angle: f32,
    // Number of sectors composition the FOV. Each sector is a triangle
    n_sectors: u32,
    // Color of the edges of the sector triangle
    color: Color,
    // Alpha replacement to color for the sector 'background'
    sector_alpha: f32,
}

/// The GPU representation of the uniform data of a [`ColorMaterial`].
#[derive(Clone, Default, AsStd140)]
pub struct FovViewerMaterialUniformData {
    pub fov_angle: f32,
    pub n_sectors: u32,
    pub color: Vec4,
    pub sector_alpha: f32,
}

#[derive(Clone)]
pub struct GpuFovViewerMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for FovViewerMaterial {
    type ExtractedAsset = FovViewerMaterial;
    type PreparedAsset = GpuFovViewerMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let fov_uniform_data = FovViewerMaterialUniformData {
            fov_angle: extracted_asset.fov_angle,
            n_sectors: extracted_asset.n_sectors,
            color: Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32()),
            sector_alpha: extracted_asset.sector_alpha,
        };
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: fov_uniform_data.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuFovViewerMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}
impl Material2d for FovViewerMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("fov_viewer.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("fov_viewer.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(
                        FovViewerMaterialUniformData::std140_size_static() as u64,
                    ),
                },
                count: None,
            }],
            label: None,
        })
    }
}
