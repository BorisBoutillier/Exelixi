use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::prelude::*;

pub struct EyeViewerPlugin;
impl Plugin for EyeViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<FovViewerMaterial>::default())
            .add_system(spawn_fov_viewer_on_selected)
            .add_system(despawn_fov_viewer_on_deselected.in_base_set(CoreSet::PostUpdate));
    }
}
/// a a FOV viewer child whenver a Selected component is added to an entity with an Eye
fn spawn_fov_viewer_on_selected(
    mut commands: Commands,
    parents: Query<(Entity, &Eye), Added<Selected>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FovViewerMaterial>>,
) {
    for (parent, eye) in parents.iter() {
        let fov_viewer = commands
            .spawn(MaterialMesh2dBundle {
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
fn despawn_fov_viewer_on_deselected(
    mut commands: Commands,
    fov_viewers: Query<&Handle<FovViewerMaterial>>,
    children_query: Query<&Children>,
    mut deselected: RemovedComponents<Selected>,
) {
    for entity in deselected.iter() {
        for child in children_query.iter_descendants(entity) {
            if fov_viewers.contains(child) {
                commands.entity(child).despawn();
            }
        }
    }
}

#[derive(Component, Debug, Clone, TypeUuid, AsBindGroup)]
#[uuid = "516c3ab4-6a1c-4e7d-9795-6161ca083a1d"]
struct FovViewerMaterial {
    // Total angle of the FOV to show, will be show from -fov_angle/2 to fov_anglel2
    #[uniform(0)]
    fov_angle: f32,
    // Number of sectors composition the FOV. Each sector is a triangle
    #[uniform(0)]
    n_sectors: u32,
    // Color of the edges of the sector triangle
    #[uniform(0)]
    color: Color,
    // Alpha replacement to color for the sector 'background'
    #[uniform(0)]
    sector_alpha: f32,
}

impl Material2d for FovViewerMaterial {
    fn fragment_shader() -> ShaderRef {
        "fov_viewer.wgsl".into()
    }
}
