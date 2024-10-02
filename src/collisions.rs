//use std::fmt::format;
//use wasm_bindgen::JsValue;
//use web_sys::console;
use crate::Player;
use std::collections::HashMap;
use crate::map;
use crate::Tile;

pub enum UpDown {
    Up,
    Down
}
pub fn tile_collision(tuple: (usize, usize), collision_map: &HashMap<(usize, usize), Tile>) -> Option<&map::Tile> {
    collision_map.get(&tuple)
}
pub fn manage_collision(
    ntuple: ((usize, usize), (usize, usize), (usize, usize), f64, f64, f64, f64, f64, f64), 
    collision_map: &HashMap<(usize, usize), Tile>,
    player: &mut Player,
    up_down: UpDown
) {
    let (
        main_tile,
        second_tile,
        third_tile,
        off_tile_x,
        off_tile_y,
        off_tile_x_intersection,
        off_tile_y_intersection,
        off_player_x,
        off_player_y
    ) = ntuple;

    if tile_collision(main_tile, &collision_map).is_some() {
        //console::log_1(&JsValue::from_str(""));
        if let Some(t) = tile_collision(second_tile, &collision_map) {
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y
        }
        if let Some(t) = tile_collision(third_tile, &collision_map) {
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x
        }
        if tile_collision(second_tile, &collision_map).is_none() && tile_collision(third_tile, &collision_map).is_none() {
            //console::log_1(&JsValue::from_str("calc inter"));
            if let Some(t) = tile_collision(main_tile, &collision_map) {
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
                } else {
                    player.velocity.x = 0.;
                    player.position.x = t.position.x + off_tile_x;
                }
            }
        }
    } else {
        if let Some(t) = tile_collision(second_tile, &collision_map) {
            //console::log_1(&JsValue::from_str("top right ---"));
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y
        }
        if let Some(t) = tile_collision(third_tile, &collision_map) {
            //console::log_1(&JsValue::from_str("top third_tile ---"));
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x
        }
    }
}

pub fn manage_player_collision_with_tile(player: &mut Player, collision_map: &HashMap<(usize, usize), Tile>) {
    //tiles around the player
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

    let mut d_cases = HashMap::new();
    // off_tile_x, off_tile_y, off_tile_x_intersection, off_tile_y_intersection, off_player_x, off_player_y
    d_cases.insert(String::from("up-left"), (top_left, top_right, bottom_left, 50.0, 50.0, 50., 50.0, 0., 0.));
    d_cases.insert(String::from("up-right"), (top_right, top_left, bottom_right, -50.1, 50.0, 0., 50.0, 50., 0.));
    d_cases.insert(String::from("down-left"), (bottom_left, bottom_right, top_left, 50., -50.1, 50., 0., 0., 50.));
    d_cases.insert(String::from("down-right"), (bottom_right, bottom_left, top_right, -50.1, -50.1, 0., 0., 50., 50.));

    if player.velocity.y <= 0. {
        if player.velocity.x < 0. {
            manage_collision(*d_cases.get("up-left").unwrap(), collision_map, player,UpDown::Up)
        } else {
            manage_collision(*d_cases.get("up-right").unwrap(), collision_map, player,UpDown::Up);
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            manage_collision(*d_cases.get("down-left").unwrap(), collision_map, player,UpDown::Down);
        } else {
            manage_collision(*d_cases.get("down-right").unwrap(), collision_map, player, UpDown::Down);
        }
    }
}
