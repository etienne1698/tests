use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 7.0;

pub fn move_with_keyboard<T: Component>(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController, With<T>>,
) {
    for mut controller in controllers.iter_mut() {
        if keys.pressed(KeyCode::ArrowUp) {
            controller.translation = Some(Vec3::new(0.0, 0.0, -SPEED * time.delta_secs()));
        }
        if keys.pressed(KeyCode::ArrowDown) {
            controller.translation = Some(Vec3::new(0.0, 0.0, SPEED * time.delta_secs()));
        }

        if keys.pressed(KeyCode::ArrowRight) {
            controller.translation = Some(Vec3::new(SPEED * time.delta_secs(), 0.0, 0.0));
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            controller.translation = Some(Vec3::new(-SPEED * time.delta_secs(), 0.0, 0.0));
        }
    }
}
