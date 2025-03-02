use std::collections::HashMap;
use crate::{play, Player, Tile};
use crate::collisions::types::LeftRight;

pub fn player_move(
    player: &mut Player, 
    delta: f64,
    collision_map: &mut HashMap<(usize,usize), Tile>
) {
    let brake = if player.moves.airborne { 0.06 } else { 0.5 };
    let acceleration = 0.3;
    if player.moves.right {
        if player.velocity.x < player.horizontal_velocity {
            player.velocity.x += acceleration 
        }
    } else if player.moves.left{
        if player.velocity.x > -player.horizontal_velocity {
            player.velocity.x -= acceleration
        }
    } else if !player.is_hanging {
        if player.velocity.x >= brake {
            player.velocity.x -= brake
        } else if player.velocity.x <= -brake {
            player.velocity.x += brake
        } else {
            player.velocity.x = 0.
        }
    } else {
        player.velocity.x = 0.
    } 
    //player.velocity.y = if player.moves.down { 4.0 } else if player.moves.up { -4.0 } else { 0. };
    if !player.is_clinging {
        if player.velocity.x > 0. {
            player.facing_right = true;
            player.facing_left = false;
        }
        if player.velocity.x < 0. {
            player.facing_right = false;
            player.facing_left = true;
        }
    }
    if player.moves.jump {
        player.moves.jump = false;
        player.velocity.y = player.jump_velocity; //-10.1
    }
    if player.moves.stop_jump {
        if player.velocity.y <= 0.0 {
            player.velocity.y += 0.3//3
        } 
    }
    if player.velocity.y < player.max_fall_velocity {
        player.velocity.y += player.gravity / delta
    }
    if player.wants_to_cling && player.can_cling != LeftRight::None {
        player.is_clinging = true;
        if !player.played_clinging_sound {
            play("cling");
            player.played_clinging_sound = true;
        }
    }
    if player.is_clinging {
        if let Some(tile) = player.clinging_tile_coord {
            if collision_map.get(&tile).is_some() {
                player.velocity.y = 0.;
                player.velocity.x = 0.;
            } else {
                player.is_clinging = false;
                player.played_clinging_sound = false;
                //player.can_cling = LeftRight::None;
            }
        }
    }
    if !player.is_hanging {
        player.position.x += player.velocity.x / delta;
        player.position.y += player.velocity.y / delta;
    } else {
        player.velocity.y = 0.;
        player.position.x += player.velocity.x / delta;
    }
}
