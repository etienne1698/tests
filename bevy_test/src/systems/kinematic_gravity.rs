use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = -9.81;

pub fn kinematic_gravity<T: Component>(
    time: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController, With<T>>,
) {
    for mut controller in controllers.iter_mut() {
        let mut translation: Vec3 = controller.translation.unwrap_or(Vec3::ZERO);
        translation.y += GRAVITY * time.delta_secs();
        controller.translation = Some(translation);
    }
}
