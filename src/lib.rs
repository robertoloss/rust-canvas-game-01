mod map;
mod utils;
mod collisions;
mod player;
mod enemies;
pub mod draw;
use enemies::enemies_move::enemies_move;
use enemies::types::EnemyTrait;
use map::restore_sand_tiles::restore_sand_tiles;
use player::player_can_still_hang::player_can_still_hang;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use std::{collections::HashMap, sync::Mutex};
use crate::player::types::*;
use crate::utils::utils::*;
use crate::player::player_move::*;
use crate::map::map::*;
use crate::map::map_move::*;
use crate::map::generate_map_collisions::*;
use crate::draw::main_draw::*;
use crate::collisions::lethal_tile_collision::*;
use crate::collisions::manage_player_collision_with_tile::manage_player_collision_with_tile;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
    static ref LETHAL_TILES: Mutex<Vec<Tile>> = Mutex::new(vec![]);
    static ref ENEMIES: Mutex<Vec<Box<dyn EnemyTrait>>> = Mutex::new(vec![]);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = SoundManager)]
    fn play(sound_name: &str);
}
pub fn play_sound(sound_name: &str) {
    play(sound_name); 
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
    if delta == 0. { return Ok(()) }

    enemies_move();

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
        if lethal_tile_collision(&tile, &player) {
            player.is_dead = true;
        }
    }
    restore_sand_tiles(
        &mut player,
        &mut collision_map
    );

    manage_player_collision_with_tile(&mut(*player), &mut collision_map); 

    if !player_can_still_hang(&mut player, &mut collision_map) {
        player.is_hanging = false
    }

    if !all_images_present(&player.images) { return Ok(()) }
    if !all_sprite_sheets_present(&player.sprite_sheets) { return Ok(()) }

    let res = main_draw(
        &mut collision_map,
        &mut player,
    );
    res
}

