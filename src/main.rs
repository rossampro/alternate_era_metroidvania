mod components;
mod plugins;
mod systems;
use crate::plugins::hello_plugin::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
