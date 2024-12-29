use bevy::prelude::*;
use bevy::ecs::component::Component;

pub fn follow_camera<TToFollow: Component, TFollow: Component>(
    to_follow_query: Query<&Transform, With<TToFollow>>,
    mut camera_query: Query<&mut Transform, (With<TFollow>, Without<TToFollow>)>,
) {
    // Récupère la position du joueur
    if let Ok(to_follow_transform) = to_follow_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            // Positionne la caméra à une distance relative par rapport au joueur
            camera_transform.translation = to_follow_transform.translation + Vec3::new(0.0, 4.0, 8.0);

            // Oriente la caméra pour qu'elle regarde toujours vers le joueur
            camera_transform.look_at(to_follow_transform.translation, Vec3::Y);
        }
    }
}
