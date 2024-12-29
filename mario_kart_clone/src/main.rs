mod plugins;
mod components;
mod systems;

use bevy::prelude::*;
use plugins::basic_scene_plugin::*;
use plugins::player_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasicScenePlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
