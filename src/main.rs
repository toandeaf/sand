mod player;
mod world;

use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .run();
}
