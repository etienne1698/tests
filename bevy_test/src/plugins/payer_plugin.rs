use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::components::{FollowCamera, Player};
use crate::resources::PlayerResource;
use crate::systems::{follow_camera, kinematic_gravity, move_with_keyboard};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerResource {
        run: asset_server.load(GltfAssetLabel::Animation(1).from_asset("3d/player.glb")),
    });

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

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 4.0),
        FollowCamera,
    ));
}

fn setup_animations(
    mut has_setup: Local<bool>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    player_resource: Res<PlayerResource>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if *has_setup {
        return;
    }
    for (entity, mut player) in &mut players {
        // The name of the entity in the GLTF scene containing the AnimationPlayer for our morph targets is "Main"
       

        let (graph, animation) = AnimationGraph::from_clip(player_resource.run.clone());
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(graphs.add(graph)));

        player.play(animation).repeat();
        *has_setup = true;
    }
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
        ).add_systems(Update, setup_animations);
        
    }
}
