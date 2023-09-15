use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;
#[derive(Component)]
pub struct MouthViewer;

pub struct MouthViewerPlugin;
impl Plugin for MouthViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_mouth_viewer_on_selected)
            .add_systems(PostUpdate, despawn_mouth_viewer_on_deselected);
    }
}
/// a a FOV viewer child whenver a Selected component is added to an entity with an Eye
fn spawn_mouth_viewer_on_selected(
    mut commands: Commands,
    parents: Query<(Entity, &Mouth), Added<Selected>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (parent, mouth) in parents.iter() {
        let mouth_viewer = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle {
                            radius: mouth.reach,
                            vertices: 32,
                        }))
                        .into(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.5),
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.6))),
                    ..Default::default()
                },
                MouthViewer,
            ))
            .id();
        commands.entity(parent).add_child(mouth_viewer);
    }
}
fn despawn_mouth_viewer_on_deselected(
    mut commands: Commands,
    children_query: Query<&Children>,
    mut deselected: RemovedComponents<Selected>,
    mouth_viewers: Query<(), With<MouthViewer>>,
) {
    for entity in deselected.iter() {
        for child in children_query.iter_descendants(entity) {
            if mouth_viewers.contains(child) {
                commands.entity(child).despawn();
            }
        }
    }
}
