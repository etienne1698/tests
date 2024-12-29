mod plugins;
mod components;

use bevy::prelude::*;
use plugins::hello_plugin::*;
use plugins::basic_scene_plugin::*;
use plugins::player_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(BasicScenePlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
