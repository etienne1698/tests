use bevy::prelude::*;

use crate::components::{FollowCamera, Player};

const SPEED: f32 = 0.1;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 4.0),
        FollowCamera,
    ));

    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/player.glb"))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
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


fn follow_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Player>)>,
) {
    // Récupère la position du joueur
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            // Positionne la caméra à une distance relative par rapport au joueur
            camera_transform.translation = player_transform.translation + Vec3::new(0.0, 4.0, 8.0);

            // Oriente la caméra pour qu'elle regarde toujours vers le joueur
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_player, follow_camera));
    }
}
