// Have the Transform components follow the Position components all simulation entities
use crate::prelude::*;
pub fn organism_transform_update(
    mut query: Query<(&mut Transform, &Position), Or<(Added<Transform>, Changed<Position>)>>,
) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.translation.z = 10.0;
        transform.rotation = Quat::from_axis_angle(Vec3::Z, position.angle());
    }
}

// Insert the appropriate SpriteBundle and Transform for all simulation entities.
pub fn show_organism(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_organisms: Query<(Entity, &Organism), Added<Organism>>,
) {
    for (entity, organism) in new_organisms.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(organism.kind.get_sprite_size()),
                color: Color::hsl(organism.kind.get_hue(), 0.8, 0.5),
                ..Default::default()
            },
            texture: asset_server.load(organism.kind.get_sprite_file()),
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

impl OrganismKind {
    pub fn get_hue(&self) -> f32 {
        match self {
            OrganismKind::Plant => 120.0,
            OrganismKind::Herbivore => 300.0,
        }
    }
    pub fn get_sprite_file(&self) -> String {
        match self {
            OrganismKind::Plant => "food.png",
            OrganismKind::Herbivore => "bird.png",
        }
        .into()
    }
    pub fn get_sprite_size(&self) -> Vec2 {
        match self {
            OrganismKind::Plant => Vec2::new(8.0, 8.0),
            OrganismKind::Herbivore => Vec2::new(20.0, 20.0),
        }
    }
}
