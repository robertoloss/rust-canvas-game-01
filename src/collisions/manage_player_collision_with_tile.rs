use crate::{Player,Tile};
use std::collections::HashMap;
use crate::collisions::types::{LeftRight,UpDown};
use crate::collisions::manage_collision::manage_collision;


pub fn manage_player_collision_with_tile(player: &mut Player, collision_map: &HashMap<(usize, usize), Tile>) {
    let tile_size = player.tile_size;
    //tiles around the player
    let top_right = (
        ((player.position.x + player.velocity.x + tile_size - player.hitbox.right) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y) / tile_size).floor() as usize
    );
    let bottom_right = (
        ((player.position.x + player.velocity.x + tile_size - player.hitbox.right) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
    );
    let top_left = (
        ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y) / tile_size).floor() as usize
    );
    let bottom_left = (
        ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
    );

    if player.velocity.x == 0. && player.velocity.y == 0. { return }

    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            manage_collision(
                (top_left, top_right, bottom_left),
                collision_map, 
                player,
                UpDown::Up, 
                LeftRight::Left
            )
        } else {
            manage_collision(
                (top_right, top_left, bottom_right),
                collision_map, 
                player,
                UpDown::Up, 
                LeftRight::Right
            );
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            manage_collision(
                (bottom_left, bottom_right, top_left),
                collision_map, 
                player,
                UpDown::Down,
                LeftRight::Left
            );
        } else {
            manage_collision(
                (bottom_right, bottom_left, top_right),
                collision_map, 
                player, 
                UpDown::Down,
                LeftRight::Right
            );
        }
    }
}
