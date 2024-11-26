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
    //off_tile_x = how much to the x will be the player repositioned
    // (....3 tiles...., off_tile_x, off_tile_y, off_tile_x_intersection, off_tile_y_intersection, off_player_x, off_player_y)
    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            manage_collision(
                (top_left, top_right, bottom_left, tile_size, tile_size, player.hitbox.left, 0.),
                collision_map, 
                player,
                UpDown::Up, 
                LeftRight::Left
            )
        } else {
            manage_collision(
                (top_right, top_left, bottom_right, 0., tile_size, tile_size -player.hitbox.right, 0.),
                collision_map, 
                player,
                UpDown::Up, 
                LeftRight::Right
            );
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            manage_collision(
                (bottom_left, bottom_right, top_left, tile_size, 0., player.hitbox.left, tile_size),
                collision_map, 
                player,
                UpDown::Down,
                LeftRight::Left
            );
        } else {
            manage_collision(
                (bottom_right, bottom_left, top_right, 0., 0., tile_size - player.hitbox.right, tile_size),
                collision_map, 
                player, 
                UpDown::Down,
                LeftRight::Right
            );
        }
    }
}
