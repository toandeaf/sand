use crate::player::{AnimationTimer, Player};
use bevy::prelude::*;
use std::ops::Range;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn range(&self) -> Range<usize> {
        match *self {
            Direction::Up => 0..8,
            Direction::Down => 9..17,
            Direction::Left => 18..26,
            Direction::Right => 27..35,
        }
    }
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_bundle: Query<
        (&mut Transform, &mut AnimationTimer, &mut TextureAtlasSprite),
        With<Player>,
    >,
) {
    for (mut transform, mut timer, mut sprite) in &mut player_bundle {
        timer.tick(time.delta());
        if timer.just_finished() {
            if keyboard_input.pressed(KeyCode::W) {
                transform.translation.y += 1000. * time.delta_seconds();
                sprite.index = calculate_sprite(Direction::Up, &sprite.index);
            } else if keyboard_input.pressed(KeyCode::S) {
                transform.translation.y -= 1000. * time.delta_seconds();
                sprite.index = calculate_sprite(Direction::Down, &sprite.index);
            } else if keyboard_input.pressed(KeyCode::A) {
                transform.translation.x -= 1000. * time.delta_seconds();
                sprite.index = calculate_sprite(Direction::Left, &sprite.index);
            } else if keyboard_input.pressed(KeyCode::D) {
                transform.translation.x += 1000. * time.delta_seconds();
                sprite.index = calculate_sprite(Direction::Right, &sprite.index);
            }
        }
    }
}

fn calculate_sprite(direction: Direction, sprite_index: &usize) -> usize {
    if !direction.range().contains(sprite_index) {
        direction.range().start
    } else {
        sprite_index + 1
    }
}
