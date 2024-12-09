use std::collections::HashMap;
use web_sys::console;

use crate::{Player, Tile};
use crate::collisions::types::{LeftRight,UpDown};
use crate::collisions::manage_collision::manage_collision;


pub fn manage_player_collision_with_tile(
    player: &mut Player, 
    collision_map: &mut HashMap<(usize, usize), Tile>
) {
    let tile_size = player.tile_size;
    //tiles around the player
    let top_right = (
        ((player.position.x + player.velocity.x + tile_size - player.hitbox.right) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y) / tile_size).floor() as usize
    );
    let top_left = (
        ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y) / tile_size).floor() as usize
    );
    let bottom_right = (
        ((player.position.x + player.velocity.x + tile_size - player.hitbox.right) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
    );
    let bottom_left = (
        ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
    );
    let bottom_left_behind = (
        ((player.position.x + player.velocity.x + (6. * 3.)) / tile_size).floor() as usize,
        ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
    );

    if player.velocity.x == 0. && player.velocity.y == 0. { return }

    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            manage_collision(
                collision_map,
                (top_left, top_right, bottom_left),
                player,
                UpDown::Up, 
                LeftRight::Left
            )
        } else {
            manage_collision(
                collision_map,
                (top_right, top_left, bottom_right),
                player,
                UpDown::Up, 
                LeftRight::Right
            );
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            manage_collision(
                collision_map,
                (bottom_left, bottom_right, top_left),
                player,
                UpDown::Down,
                LeftRight::Left
            );
        } else 
            //if player.velocity.x > 0. 
        {
            manage_collision(
                collision_map,
                (bottom_right, bottom_left, top_right),
                player, 
                UpDown::Down,
                LeftRight::Right
            );
        } 
        //else if player.facing_left {
        //    manage_collision(
        //        collision_map,
        //        (bottom_left, bottom_right, top_left),
        //        player,
        //        UpDown::Down,
        //        LeftRight::Left
        //    );
        //} else {
        //    manage_collision(
        //        collision_map,
        //        (bottom_right, bottom_left_behind, top_right),
        //        player, 
        //        UpDown::Down,
        //        LeftRight::Right
        //    );
        //}
    }
}
