use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 0.1;

pub fn move_with_keyboard<T: Component>(
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
        if keys.pressed(KeyCode::ArrowUp) {
            controller.translation = Some(Vec3::new(0.0, 0.0, -SPEED));
        }
        if keys.pressed(KeyCode::ArrowDown) {
            controller.translation = Some(Vec3::new(0.0, 0.0, SPEED));
        }

        if keys.pressed(KeyCode::ArrowRight) {
            controller.translation = Some(Vec3::new(SPEED, 0.0, 0.0));
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            controller.translation = Some(Vec3::new(-SPEED, 0.0, 0.0));
        }
    }
}
