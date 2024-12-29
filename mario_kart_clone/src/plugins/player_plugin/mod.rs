use bevy::prelude::*;

use crate::components::Player;

const SPEED: f32 = 0.1;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Player
    ));
}

fn move_player(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        
        let forward = transform.rotation * Vec3::Z; 
        let forward_normalized = forward.normalize();

        let right = transform.rotation * Vec3::X;
        let right_normalized = right.normalize();

        if keys.pressed(KeyCode::ArrowUp) {
            transform.translation -= forward_normalized * SPEED;
            return;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            transform.translation += forward_normalized * SPEED;
            return;
        }
        
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation += right_normalized * SPEED;
            return;
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation -= right_normalized * SPEED;
            return;
        }
        
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_player);
    }
}
