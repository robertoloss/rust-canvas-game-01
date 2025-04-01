use std::collections::HashMap;
use crate::particles::types::Particle;
use crate::{Player, Tile};
use crate::collisions::types::{LeftRight,UpDown};
use crate::collisions::manage_collision::manage_collision;
use super::get_tiles_around_player::get_tiles_around_player;

pub fn manage_player_collision_with_tile(
    player: &mut Player, 
    collision_map: &mut HashMap<(usize, usize), Tile>,
    particles: &mut Vec<Particle>
) {
    let [
        top_right,
        top_left,
        bottom_right,
        bottom_left
    ] = get_tiles_around_player(player);

    if player.velocity.x == 0. && player.velocity.y == 0. { return }

    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            manage_collision(
                collision_map,
                (top_left, top_right, bottom_left),
                player,
                UpDown::Up, 
                LeftRight::Left,
                particles
            )
        } else {
            manage_collision(
                collision_map,
                (top_right, top_left, bottom_right),
                player,
                UpDown::Up, 
                LeftRight::Right,
                particles
            );
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            manage_collision(
                collision_map,
                (bottom_left, bottom_right, top_left),
                player,
                UpDown::Down,
                LeftRight::Left,
                particles
            );
        } else {
            manage_collision(
                collision_map,
                (bottom_right, bottom_left, top_right),
                player, 
                UpDown::Down,
                LeftRight::Right,
                particles
            );
        } 
    }
}
