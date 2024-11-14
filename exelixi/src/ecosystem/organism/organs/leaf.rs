use crate::ecosystem::*;

/// A leaf is an organ that will continuously create energy while present,
/// a leaf has a limited lifetime after which it disappears.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Leaf {
    // Energy produced per step
    pub energy_production: f32,
    // Number of step remaining before disappearing,
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
}
impl EnergyActor for Leaf {
    fn energy_produced(&self) -> f32 {
        self.energy_production
    }
}

pub fn leaf_processing(mut leaves: Query<&mut Leaf>) {
    for mut leaf in leaves.iter_mut() {
        if leaf.lifetime > 0 {
            leaf.lifetime -= 1;
            if leaf.lifetime == 0 {
                leaf.energy_production = 0.0;
            }
        }
    }
}
