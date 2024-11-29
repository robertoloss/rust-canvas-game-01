mod map;
mod utils;
mod collisions;
mod player;
mod draw;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::console::{self, log_1};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use std::{collections::HashMap, sync::Mutex};
use crate::player::types::*;
use crate::utils::utils::*;
use crate::player::player_move::*;
use crate::map::map::*;
use crate::map::map_move::*;
use crate::map::generate_map_collisions::*;
use crate::draw::main_draw::*;
use crate::collisions::normal_tile_collision::*;
use crate::collisions::manage_player_collision_with_tile::manage_player_collision_with_tile;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
    static ref LETHAL_TILES: Mutex<Vec<Tile>> = Mutex::new(vec![]);
}

#[wasm_bindgen]
pub fn initialize() {
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let player = PLAYER.lock().unwrap();
    *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player)).0;
    *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player)).1;
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    let delta = player.delta / 60.; //0.016 * (0.016 * 1000. * 3.3);
    if delta == 0. {
        return Ok(())
    }

    if !player.is_dead {
        player_move(
            &mut player,
            delta,
            &mut collision_map
        );
        map_move(
            &mut player,
            &mut lethal_tiles,
            num_of_tiles,
            tile_size,
            &mut collision_map,
        )
    }
    for tile in lethal_tiles.iter() {
        if normal_tile_collision(&tile, &player) {
            player.is_dead = true;
        }
    }

    let origin_x = player.map_origin.x.clone();
    let origin_y = player.map_origin.y.clone();
    let sheet = Some(player.sprite_sheets.get("sand").unwrap().clone()); 
    let tiles_to_restore = &mut player.tiles_to_restore;
    for i in (0..tiles_to_restore.len()).rev() {
        let tile= &mut player.tiles_to_restore[i];
        if tile.counter >= tile.counter_limit {
            let new_tile = Tile {
                tile_pos: Vec2usize {
                    x: tile.tile_coordinates.x - origin_x,
                    y: tile.tile_coordinates.y - origin_y,
                },
                position: Vec2 {
                    x: (tile.tile_coordinates.x - origin_x) as f64 * tile_size,
                    y: (tile.tile_coordinates.y - origin_y) as f64 * tile_size
                },
                sheet: sheet.clone(),
                touched_by_player: false
            };
            collision_map.insert(
                ( 
                    tile.tile_coordinates.x - origin_x, 
                    tile.tile_coordinates.y - origin_y
                ), 
                new_tile.clone()
            );
            tile.remove_tile = true;
        } else {
            tile.counter += 1;
        }
    }
    let tiles_to_restore2 = &mut player.tiles_to_restore;
    tiles_to_restore2.retain(|tile| {
        if tile.remove_tile {
            false
        } else {
            true
        }
    });

    manage_player_collision_with_tile(&mut(*player), &mut collision_map); 

    if !all_images_present(&player.images) { return Ok(()) }
    if !all_sprite_sheets_present(&player.sprite_sheets) { return Ok(()) }

    let res = main_draw(
        &mut collision_map,
        &mut player,
        tile_size,
        num_of_tiles,
    );
    res
}

