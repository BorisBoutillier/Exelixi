use crate::ecosystem::*;

/// A leaf is an organ that will continuously create energy while present,
/// a leaf has a limited lifetime after which it disappears.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Leaf {
    // Energy produced per step
    pub energy_production: f32,
    // Number of step remaing before disappearing,
    // It decreates every step.
    pub lifetime: u32,
}

impl Leaf {
    pub fn new(config: &LeafConfig) -> Self {
        Self {
            lifetime: config.lifetime,
            energy_production: config.energy_production,
        }
    }
    pub fn energy_cost(&self) -> f32 {
        // Negative, as this is an energy production organ
        -self.energy_production
    }
}

pub fn leaf_lifecycle(mut commands: Commands, mut leafs: Query<(Entity, &mut Leaf)>) {
    for (entity, mut leaf) in leafs.iter_mut() {
        if leaf.lifetime > 1 {
            leaf.lifetime -= 1
        } else {
            commands.entity(entity).remove::<Leaf>();
        }
    }
}
