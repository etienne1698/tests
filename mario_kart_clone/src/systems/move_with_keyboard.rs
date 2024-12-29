use bevy::prelude::*;
use bevy::ecs::component::Component;

const SPEED: f32 = 0.1;

pub fn move_with_keyboard<T: Component>(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<T>>,
) {
    for mut transform in &mut query {
        let forward = transform.rotation * Vec3::Z;
        let forward_normalized = forward.normalize();

        let right = transform.rotation * Vec3::X;
        let right_normalized = right.normalize();

        if keys.pressed(KeyCode::ArrowUp) {
            transform.translation -= forward_normalized * SPEED;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            transform.translation += forward_normalized * SPEED;
        }

        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation += right_normalized * SPEED;
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation -= right_normalized * SPEED;
        }
    }
}
