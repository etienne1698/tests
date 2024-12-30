use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerResource {
    pub run: Handle<AnimationClip>,
}