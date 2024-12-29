use crate::components::{FollowCamera, Player};
use bevy::prelude::*;

pub fn follow_camera(
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
