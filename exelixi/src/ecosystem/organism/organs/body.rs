use crate::ecosystem::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Body {
    cur_energy: f32,
    max_energy: f32,
    // Body energy consumption per step
    body_cost: f32,
}
impl Body {
    pub fn new(config: &BodyConfig) -> Self {
        Self {
            cur_energy: config.starting_energy,
            max_energy: config.maximum_energy,
            body_cost: config.body_cost,
        }
    }
    pub fn energy(&self) -> f32 {
        self.cur_energy
    }
    // Return current energy percentage in regard to maximum energy
    pub fn energy_pct(&self) -> f32 {
        self.cur_energy / self.max_energy
    }
    pub fn spend_energy(&mut self, energy: f32) {
        self.cur_energy -= energy;
    }
    pub fn add_energy(&mut self, energy: f32) {
        self.cur_energy = (self.cur_energy + energy).min(self.max_energy);
    }
    pub fn set_energy(&mut self, energy: f32) {
        self.cur_energy = energy.min(self.max_energy);
    }
    pub fn is_dead(&self) -> bool {
        self.cur_energy <= 0.0
    }
}
impl Sensor for Body {
    fn n_sensors(&self) -> usize {
        1
    }

    fn sensors(&self) -> Vec<f32> {
        vec![self.cur_energy / self.max_energy]
    }
}
impl EnergyConsumer for Body {
    fn energy_consumed(&self) -> f32 {
        self.body_cost
    }
}

pub fn body_energy_consumption(
    mut commands: Commands,
    mut bodies: Query<(Entity, &mut Body)>,
    producers: Query<(Option<&Leaf>, Option<&Mouth>), With<Body>>,
    consumers: Query<(Option<&Brain>, Option<&Eye>, Option<&Locomotion>), With<Body>>,
) {
    for (entity, mut body) in bodies.iter_mut() {
        let (a, b) = producers.get(entity).unwrap();
        if let Some(producer) = a {
            body.add_energy(producer.energy_produced());
        }
        if let Some(producer) = b {
            body.add_energy(producer.energy_produced());
        }
        let own_consumption = body.energy_consumed();
        body.spend_energy(own_consumption);
        let (a, b, c) = consumers.get(entity).unwrap();
        if let Some(consumer) = a {
            body.spend_energy(consumer.energy_consumed())
        }
        if let Some(consumer) = b {
            body.spend_energy(consumer.energy_consumed())
        }
        if let Some(consumer) = c {
            body.spend_energy(consumer.energy_consumed())
        }
        //let consumers = consumers.get(entity).unwrap();
        //for consumer in [
        //    consumers.0.map(|v| Box::new(v as &dyn EnergyConsumer)),
        //    consumers.1.map(|v| Box::new(v as &dyn EnergyConsumer)),
        //    consumers.2.map(|v| Box::new(v as &dyn EnergyConsumer)),
        //]
        //.into_iter()
        //.flatten()
        //{
        //    body.spend_energy(consumer.energy_consumed());
        //}
        if body.is_dead() {
            // We have consume more energy than we had in stock, we are dead.
            // Death consists simply in fully despawning ourself.
            commands.entity(entity).despawn_recursive();
        }
    }
}
