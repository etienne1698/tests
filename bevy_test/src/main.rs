mod plugins;

use bevy::prelude::*;
use plugins::hello_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
