use crate::prelude::*;
mod camera;
mod ui;

pub use camera::*;
pub use ui::*;

pub struct VisualizationPlugin;
impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(ui::UiPlugin {})
            .add_plugin(camera::CameraPlugin {})
            .add_system(add_visualization)
            .add_system(transform_update);
    }
}

// Have the Transform components follow the Position components all simulation entities
fn transform_update(
    mut query: Query<(&mut Transform, &Position), Or<(Added<Transform>, Changed<Position>)>>,
) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.rotation = Quat::from_axis_angle(Vec3::Z, position.angle());
    }
}

// Insert the appropriate SpriteBundle and Transform for all simulation entities.
fn add_visualization(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_foods: Query<(Entity, &Food), Added<Food>>,
    new_organisms: Query<(Entity, &Organism, Option<&Selected>), Added<Organism>>,
) {
    for (entity, _food) in new_foods.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::rgb(0.1, 0.7, 0.1),
                ..Default::default()
            },
            texture: asset_server.load("food.png"),
            ..Default::default()
        });
    }
    for (entity, _organism, selected) in new_organisms.iter() {
        let color = if selected.is_some() {
            Color::rgb(0.2, 0.9, 0.9)
        } else {
            Color::rgb(0.8, 0.3, 0.8)
        };
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                color,
                ..Default::default()
            },
            texture: asset_server.load("bird.png"),
            ..Default::default()
        });
    }
}
