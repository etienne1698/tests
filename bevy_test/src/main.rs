mod plugins;
mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use plugins::basic_scene_plugin::*;
use plugins::payer_plugin::*;
use plugins::physics_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasicScenePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}
