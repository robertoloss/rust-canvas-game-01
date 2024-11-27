use crate::{Player,Tile};
use std::collections::HashMap;
use crate::collisions::types::{LeftRight,UpDown};
use crate::collisions::get_manage_collision_params::*;

pub fn tile_collision(
    tuple: (usize, usize), 
    collision_map: &mut HashMap<(usize, usize), Tile>
) -> Option<&mut Tile> {
    collision_map.get_mut(&tuple)
}

pub fn manage_collision(
    ntuple: ((usize, usize), (usize, usize), (usize, usize)), 
    player: &mut Player,
    up_down: UpDown,
    left_right: LeftRight
) {
    let (
        corner_tile,
        next_to_corner_tile,
        opposite_y_to_corner_tile,
    ) = ntuple;
    let (
        off_tile_x,
        off_tile_y,
        off_tile_x_intersection,
        off_tile_y_intersection,
        off_player_x,
        off_player_y
    ) = get_manage_collision_parameter(up_down.clone(), left_right.clone(), player);
    
    let mut check_airborne = || {
        if let UpDown::Down = up_down {
            player.moves.airborne = false;
            player.moves.stop_jump = false;
        }
    };
    let corner_tile_hit = tile_collision(corner_tile, player.collision_map.as_mut().unwrap()).is_some();
    if corner_tile_hit {
        //console::log_1(&JsValue::from_str(""));
        if let Some(t) = tile_collision(next_to_corner_tile, player.collision_map.as_mut().unwrap()) {
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne();
            t.touched_by_player = true;
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, player.collision_map.as_mut().unwrap()) {
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            player.can_cling = left_right.clone();
            t.touched_by_player = true;
        } else {
            player.can_cling = LeftRight::None
        }
        if tile_collision(next_to_corner_tile, player.collision_map.as_mut().unwrap()).is_none() && tile_collision(opposite_y_to_corner_tile, player.collision_map.as_mut().unwrap()).is_none() {
                let t = tile_collision(corner_tile, player.collision_map.as_mut().unwrap()).unwrap();
            let m = player.velocity.y / player.velocity.x;
            let intersection_y = m * (
                (t.position.x + off_tile_x_intersection) - (player.position.x + off_player_x )
            ) + (player.position.y + off_player_y);

            let from_below_above: bool; 
            match up_down {
                UpDown::Up => from_below_above = intersection_y > t.position.y + off_tile_y_intersection,
                UpDown::Down => from_below_above = intersection_y < t.position.y + off_tile_y_intersection
            }
            if from_below_above {
                player.velocity.y = 0.;
                player.position.y = t.position.y + off_tile_y;
                check_airborne();
                t.touched_by_player = true;
            } else {
                player.velocity.x = 0.;
                player.position.x = t.position.x + off_tile_x;
                t.touched_by_player = true;
                if let UpDown::Up = up_down {
                    player.can_cling = left_right.clone();
                }
            }
        }
    } else {
        if let Some(t) = tile_collision(next_to_corner_tile, player.collision_map.as_mut().unwrap()) {
            //console::log_1(&JsValue::from_str("top right ---"));
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne();
            t.touched_by_player = true;
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, player.collision_map.as_mut().unwrap()) {
            //console::log_1(&JsValue::from_str("top opposite_y_to_corner_tile ---"));
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            t.touched_by_player = true;
            if let UpDown::Down = up_down {
            player.can_cling = left_right.clone();
            } 
        } else {
            player.can_cling = LeftRight::None;
        }
    }
}

