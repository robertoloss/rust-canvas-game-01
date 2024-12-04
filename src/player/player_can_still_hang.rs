use std::collections::HashMap;
use crate::{Player, Tile};

pub fn player_can_still_hang(
    player: &mut Player, 
    collision_map: &mut HashMap<(usize, usize), Tile>
) -> bool {
    let tile_size = player.tile_size;
    let can_still_hang = || -> bool {
        if let Some(_) = collision_map
            .get(&(
                ((player.position.x + 
                  if player.facing_right { 
                      tile_size - tile_size/2.
                  } else { 
                      tile_size/2.
                  }
                ) / tile_size).floor() as usize, 
                ((player.position.y - tile_size) / tile_size).floor() as usize
            )) {
            return true
        }
        false
    };
    if player.facing_right {
        if can_still_hang() { return true } 
        false
    } else {
        if can_still_hang() { return true } 
        false
    }
}
