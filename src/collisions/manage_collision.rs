use crate::particles::hit_ground_particles::hit_ground_particles;
use crate::particles::types::Particle;
use crate::{log_out_f, play, Player, Tile};
use std::collections::HashMap;
use crate::collisions::types::{LeftRight,UpDown};
use crate::collisions::get_manage_collision_params::*;

pub fn tile_collision(
    tuple: (usize, usize), 
    collision_map: &mut HashMap<(usize, usize), Tile>
) -> Option<&mut Tile> {
    collision_map.get_mut(&tuple)
}

fn player_can_cling(
    left_right: &LeftRight, 
    tile: &mut Tile,
    player: &mut Player
) {
    player.can_cling = left_right.clone();
    player.clinging_tile_coord = Some((
        tile.tile_pos.x,
        tile.tile_pos.y
    ))
}

pub fn manage_collision(
    collision_map: &mut HashMap<(usize, usize), Tile>,
    ntuple: ((usize, usize), (usize, usize), (usize, usize)), 
    player: &mut Player,
    up_down: UpDown,
    left_right: LeftRight,
    particles: &mut Vec<Particle>
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
    ) = get_manage_collision_parameters(up_down.clone(), left_right.clone(), player);
    
    fn check_airborne(player: &mut Player, up_down: UpDown) {
        if let UpDown::Down = up_down {
            player.moves.airborne = false;
            player.moves.stop_jump = false;
        }
    }
    fn check_hanging(player: &mut Player, up_down: UpDown, tile: &Tile) {
        if let UpDown::Up = up_down {
            if tile.hanging_tile {
                player.velocity.x = 0.;
                player.is_hanging = true;
            }
        }
    }
    let corner_tile_hit = tile_collision(corner_tile, collision_map).is_some();

    if corner_tile_hit {
        if let Some(t) = tile_collision(next_to_corner_tile, collision_map) {
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne(player, up_down.clone());
            check_hanging(player, up_down.clone(), &t);
            t.touched_by_player = true;
            match up_down {
                UpDown::Down => {
                    if !player.on_the_ground {
                        play("ground");
                        hit_ground_particles(player, particles);
                        player.on_the_ground = true;
                    } 
                }
                _ => { }
            }
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, collision_map) {
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            t.touched_by_player = true;
            player_can_cling(&left_right, t, player);
        } else {
            player.can_cling = LeftRight::None
        }
        if 
            tile_collision(next_to_corner_tile, collision_map).is_none() && 
            tile_collision(opposite_y_to_corner_tile, collision_map).is_none() 
        {
            let t = tile_collision(corner_tile, collision_map).unwrap();
            //console::log_1(&format!("{:?}", t).into());
            let m = player.velocity.y / player.velocity.x;
            let intersection_y = m * (
                (t.position.x + off_tile_x_intersection) - (player.position.x + off_player_x )
            ) + (player.position.y + off_player_y);

            let from_below_above: bool; 
            match up_down {
                UpDown::Up => from_below_above = intersection_y > t.position.y + off_tile_y_intersection,
                UpDown::Down => {
                    from_below_above = intersection_y < t.position.y + off_tile_y_intersection;
                    if !player.on_the_ground {
                        player.on_the_ground = true;
                    } 
                }
            }
            //console::log_1(&format!("from_below_above {}", from_below_above).into());
            if from_below_above {
                player.velocity.y = 0.;
                player.position.y = t.position.y + off_tile_y;
                check_airborne(player, up_down.clone());
                check_hanging(player, up_down.clone(), &t);
                t.touched_by_player = true;
            } else {
                player.velocity.x = 0.;
                player.position.x = t.position.x + off_tile_x;
                t.touched_by_player = true;
                if let UpDown::Up = up_down {
                    player_can_cling(&left_right, t, player);
                }
            }
        }
    } else {
        if let Some(t) = tile_collision(next_to_corner_tile, collision_map) {
            //console::log_1(&JsValue::from_str("top right ---"));
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y;
            check_airborne(player, up_down.clone());
            t.touched_by_player = true;
        }
        if let Some(t) = tile_collision(opposite_y_to_corner_tile, collision_map) {
            //console::log_1(&JsValue::from_str("top opposite_y_to_corner_tile ---"));
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x;
            t.touched_by_player = true;
            if let UpDown::Down = up_down {
                player_can_cling(&left_right, t, player);
            } 
        } else {
            player.can_cling = LeftRight::None;
        }
    }
}

