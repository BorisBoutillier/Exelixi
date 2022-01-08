use crate::*;

pub fn process_brain(
    mut animals: Query<(&Transform, &mut Velocity, &Eye, &Brain, Option<&Selected>)>,
    food_transforms: Query<&Transform, With<Food>>,
) {
    let food_transforms = food_transforms.iter().collect::<Vec<_>>();
    animals.for_each_mut(
        |(animal_transform, mut animal_velocity, animal_eye, animal_brain, selected)| {
            let vision = animal_eye.process_vision(animal_transform, &food_transforms);
            let response = animal_brain.nn.propagate(&vision);
            let linear_accel = (response[0].clamp(0.0, 2.0) - 1.0) * V_LINEAR_ACCEL;
            let angular = (response[1].clamp(0.0, 2.0) - 1.0) * V_ANGULAR_MAX;
            if selected.is_some() {
                //println!("VISION {:?}", vision);
                //println!(
                //    "Response {:.3} {:.3} , {:.3} / {:.3}",
                //    response[0], response[1], linear_accel, angular
                //);
            }
            animal_velocity.linear =
                (animal_velocity.linear + linear_accel).clamp(V_LINEAR_MIN, V_LINEAR_MAX);
            animal_velocity.angular = angular;
        },
    );
}
