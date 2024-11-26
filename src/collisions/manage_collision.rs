use crate::{Player,Tile};
use std::collections::HashMap;
use crate::collisions::types::{LeftRight,UpDown};

pub fn tile_collision(
    tuple: (usize, usize), 
    collision_map: &HashMap<(usize, usize), Tile>
) -> Option<&Tile> {
    collision_map.get(&tuple)
}

pub fn manage_collision(
    ntuple: ((usize, usize), (usize, usize), (usize, usize), f64, f64, f64, f64, f64, f64), 
    collision_map: &HashMap<(usize, usize), Tile>,
    player: &mut Player,
    up_down: UpDown,
    left_right: LeftRight
) {
    let (
        corner_tile,
        next_to_corner_tile,
        opposite_y_to_corner_tile,
        off_tile_x,
        off_tile_y,
        off_tile_x_intersection,
        off_tile_y_intersection,
        off_player_x,
        off_player_y
    ) = ntuple;
    
    let mut check_airborne = || {
        if let UpDown::Down = up_down {
            player.moves.airborne = false;
            player.moves.stop_jump = false;
        }
    };

    if let Some(t) = tile_collision(corner_tile, &collision_map) {
        //console::log_1(&JsValue::from_str(""));
        if let Some(t) = tile_collision(next_to_corner_tile, &collision_map) {
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne()
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, &collision_map) {
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            player.can_cling = left_right.clone();
        } else {
            player.can_cling = LeftRight::None
        }
        if tile_collision(next_to_corner_tile, &collision_map).is_none() && tile_collision(opposite_y_to_corner_tile, &collision_map).is_none() {
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
                check_airborne()
            } else {
                player.velocity.x = 0.;
                player.position.x = t.position.x + off_tile_x;
                if let UpDown::Up = up_down {
                    player.can_cling = left_right.clone();
                }
            }
        }
    } else {
        if let Some(t) = tile_collision(next_to_corner_tile, &collision_map) {
            //console::log_1(&JsValue::from_str("top right ---"));
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne()
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, &collision_map) {
            //console::log_1(&JsValue::from_str("top opposite_y_to_corner_tile ---"));
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            if let UpDown::Down = up_down {
            player.can_cling = left_right.clone();
            } 
        } else {
            player.can_cling = LeftRight::None;
        }
    }
}

