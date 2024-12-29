use bevy::prelude::*;

mod systems;

use systems::follow_camera::follow_camera;
use systems::move_player::move_player;

use crate::components::{FollowCamera, Player};

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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_player, follow_camera).chain());
    }
}
