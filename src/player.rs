mod player_movement;

use crate::player::player_movement::movement;
use bevy::app::App;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(movement);
    }

    fn name(&self) -> &str {
        "player_plugin"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("walk.png");

    let texture_atlas_walk =
        TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 60.0), 9, 4, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas_walk);

    commands.spawn((
        Player,
        Name("Ahman".to_string()),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
    ));
}
