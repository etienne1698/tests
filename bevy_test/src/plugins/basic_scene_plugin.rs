use bevy::prelude::*;
use rand::Rng;

use bevy_rapier3d::prelude::*;

fn generate_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tree_asset_3d = asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/tree.glb"));

    let mut rng = rand::thread_rng();

    for _ in 0..15 {
        let x = rng.gen_range(-50.0..50.0); 
        let z = rng.gen_range(-50.0..50.0);
        let y = 0.0; 

        commands.spawn((
            SceneRoot(tree_asset_3d.clone()), 
            Transform::from_xyz(x, y, z),
        ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_asset_3d =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/basic_scene.glb"));

    commands.spawn((
        SceneRoot(player_asset_3d),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(100.0, 1.0, 100.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    generate_trees(commands, asset_server);
}

pub struct BasicScenePlugin;
impl Plugin for BasicScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
