use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_camera);
    }

    fn name(&self) -> &str {
        "world_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
