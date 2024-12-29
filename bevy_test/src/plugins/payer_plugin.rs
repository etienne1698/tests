use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::components::{FollowCamera, Player};
use crate::systems::{follow_camera, move_with_keyboard, kinematic_gravity};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 4.0),
        FollowCamera,
    ));

    commands.spawn((
        Player,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/player.glb"))),
        Transform::from_xyz(0.0, 40.0, 0.0),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            ..KinematicCharacterController::default()
        },
    ));
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                move_with_keyboard::<Player>,
                follow_camera::<Player, FollowCamera>,
                kinematic_gravity::<Player>,
            )
                .chain(),
        );
    }
}
