use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::Player;
use crate::map;
use crate::Tile;


pub fn tile_collision(tuple: (usize, usize), collision_map: &HashMap<(usize, usize), Tile>) -> (bool,Option<&map::Tile>) {
    (collision_map.contains_key(&tuple),collision_map.get(&tuple))
}

pub fn manage_player_collision_with_tile(player: &mut Player, collision_map: &HashMap<(usize, usize), Tile>) {
    let top_right = (
        ((player.position.x + player.velocity.x + 50.0) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y) / 50.0).floor() as usize
    );
    let bottom_right = (
        ((player.position.x + player.velocity.x + 50.0) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y + 50.0) / 50.0).floor() as usize
    );
    let top_left = (
        ((player.position.x + player.velocity.x) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y) / 50.0).floor() as usize
    );
    let bottom_left = (
        ((player.position.x + player.velocity.x) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y + 50.0) / 50.0).floor() as usize
    );

    if player.velocity.x == 0. && player.velocity.y == 0. { return }

    if player.velocity.y == 0. {
        if player.velocity.x > 0. && 
            tile_collision(top_right, &collision_map).0 || tile_collision(bottom_right, &collision_map).0 
        {
            player.velocity.x = 0.;
            if tile_collision(top_right, &collision_map).0 {
                console::log_1(&JsValue::from_str("right"));
                let tile = tile_collision(top_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x - 50.1;
                }
            } else {
                console::log_1(&JsValue::from_str("right"));
                let tile = tile_collision(bottom_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x - 50.1;
                }

            }
        } else if tile_collision(top_left, &collision_map).0 || tile_collision(bottom_left, &collision_map).0  {
            player.velocity.x = 0.;
            if tile_collision(top_left, &collision_map).0 {
                console::log_1(&JsValue::from_str("left"));
                let tile = tile_collision(top_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x + 50.0;
                }
            } else {
                console::log_1(&JsValue::from_str("left"));
                let tile = tile_collision(bottom_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x + 50.0;
                }

            }
            
        }
    } else if player.velocity.x == 0. {
        if player.velocity.y > 0. && 
            tile_collision(bottom_left, &collision_map).0 || tile_collision(bottom_right, &collision_map).0 
        {
            player.velocity.y = 0.;
            if tile_collision(bottom_left, &collision_map).0 {
                console::log_1(&JsValue::from_str("bottom"));
                let tile = tile_collision(bottom_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.y = t.position.y - 50.1;
                }
            } else {
                console::log_1(&JsValue::from_str("bottom"));
                let tile = tile_collision(bottom_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.y = t.position.y - 50.1;
                }

            }
        }
        if player.velocity.y < 0. && 
            tile_collision(top_left, &collision_map).0 || tile_collision(top_right, &collision_map).0 
        {
            player.velocity.y = 0.;
            if tile_collision(top_left, &collision_map).0 {
                console::log_1(&JsValue::from_str("top"));
                let tile = tile_collision(top_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.y = t.position.y + 50.0;
                }
            } else {
                console::log_1(&JsValue::from_str("top"));
                let tile = tile_collision(top_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.y = t.position.y + 50.0;
                }

            }
        }
    }
}
