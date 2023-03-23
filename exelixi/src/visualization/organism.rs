// Have the Transform components follow the Position components all simulation entities
use crate::prelude::*;
pub fn organism_transform_update(
    mut query: Query<(&mut Transform, &Position), Or<(Added<Transform>, Changed<Position>)>>,
) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.rotation = Quat::from_axis_angle(Vec3::Z, position.angle());
    }
}

// Insert the appropriate SpriteBundle and Transform for all simulation entities.
pub fn show_organism(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_foods: Query<(Entity, &Food), Added<Food>>,
    new_organisms: Query<(Entity, &Organism), Added<Organism>>,
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
    for (entity, _organism) in new_organisms.iter() {
        let color = Color::rgb(0.8, 0.3, 0.8);
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

// Organism sprite color lightness based on body energy pct.
pub fn sprite_lightness_from_body(mut query: Query<(&mut Sprite, &Body)>) {
    for (mut sprite, body) in query.iter_mut() {
        let [h, s, _l, a] = sprite.color.as_hsla_f32();
        let l = 0.1 + body.energy_pct().sqrt() * 0.7; // Keep in [0.1 .. 0.8 ]
        sprite.color = Color::hsla(h, s, l, a);
    }
}
