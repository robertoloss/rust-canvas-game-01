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

    let up_down = if player.velocity.y <= 0. {
        UpDown::Up
    } else {
        UpDown::Down
    };
    let left_right = if player.velocity.x < 0. {
        LeftRight::Left
    } else {
        LeftRight::Right
    };

    let mut collision_tiles = ((0,0),(0,0),(0,0));

    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            collision_tiles = (top_left, top_right, bottom_left)
        } else {
            collision_tiles = (top_right, top_left, bottom_right)
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            collision_tiles = (bottom_left, bottom_right, top_left)
        } else {
            collision_tiles = (bottom_right, bottom_left, top_right)
        } 
    }

    manage_collision(
        collision_map,
        collision_tiles,
        player,
        up_down,
        left_right,
        particles
    );
}
