use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("3d/basic_scene.glb")),
    ),));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

pub struct BasicScenePlugin;
impl Plugin for BasicScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
