mod map;
mod particles;
mod utils;
mod collisions;
mod player;
mod enemies;
pub mod draw;
use enemies::enemies_check_collision::enemies_check_collisions;
use enemies::enemies_move::enemies_move;
use enemies::types::EnemyTrait;
use map::restore_sand_tiles::restore_sand_tiles;
use particles::types::Particle;
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
    static ref PARTICLES: Mutex<Vec<Particle>> = Mutex::new(vec![]);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = SoundManager)]
    fn play(sound_name: &str);

    #[wasm_bindgen(js_namespace = window )]
    fn get_random(min: f64, max: f64) -> f64;

    #[wasm_bindgen(js_namespace = window )]
    fn get_random_int(min: u32, max: u32) -> u32;
}

#[wasm_bindgen]
pub fn initialize() {
    set_panic_hook();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let player = PLAYER.lock().unwrap();

    *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), &mut enemies).0;
    *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), &mut enemies).1;

}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    if !all_images_present(&player.images) { return Ok(()) }
    if !all_sprite_sheets_present(&player.sprite_sheets) { return Ok(()) }

    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let mut particles = PARTICLES.lock().unwrap();

    particles
        .iter_mut()
        .for_each(|particle| {
            particle.moves();
        });
    particles
        .retain(|p| p.active);

    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    let delta = player.delta / 60.; //0.016 * (0.016 * 1000. * 3.3);
    if delta == 0. { return Ok(()) }

    enemies_move(&mut enemies);
    
    if collision_map.len() == 0 { 
        log_out_f("coll map 0"); 
        drop(collision_map);
        drop(lethal_tiles);
        drop(enemies);
        drop(particles);
        drop(player);
        initialize();
        return Ok(()) 
    };

    if !player.is_dead {
        enemies_check_collisions(&mut player, &mut enemies);
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
            &mut enemies
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


    let res = main_draw(
        &mut collision_map,
        &mut player,
        &mut enemies,
        &mut particles
    );
    res
}

