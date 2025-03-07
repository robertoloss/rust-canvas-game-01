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
use particles::manage_particles::manage_particles;
use particles::types::Particle;
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

    #[wasm_bindgen(js_namespace = window )]
    fn is_game_paused() -> bool;
    
    #[wasm_bindgen(js_namespace = window )]
    fn screen_size() -> u32;
}

#[wasm_bindgen]
pub fn initialize() {
    set_panic_hook();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let player = PLAYER.lock().unwrap();

    (*collision_map,*lethal_tiles) = generate_map_collisions(
        player.map_origin.x, 
        player.map_origin.y, 
        &(*player), 
        &mut enemies,
        true
    );
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let mut particles = PARTICLES.lock().unwrap();
    
    let mut delta = (player.delta / 60.).clamp(1.1, 1.2);
    if screen_size() < 800 { 
        delta = delta.clamp(2.4, 2.6) 
    };

    if !all_images_present(&player.images) { return Ok(()) }
    if !all_sprite_sheets_present(&player.sprite_sheets) { return Ok(()) }
    if delta == 0. { return Ok(()) }
    if is_game_paused() { return Ok(()) }

    //if collision_map.len() == 0 { 
    //    log_out_f("coll map 0"); 
    //    log_out_f(player.map_origin.x);
    //    log_out_f(player.map_origin.y);
    //    if player.map_origin.x == 0 && player.map_origin.y == 0 {
    //        return Ok(())
    //    }
    //    player.map_origin.x = 0;
    //    player.map_origin.y = 0;
    //    player.position = player.position_spawn.clone();
    //    drop(collision_map);
    //    drop(lethal_tiles);
    //    drop(enemies);
    //    drop(particles);
    //    drop(player);
    //    initialize();
    //    return Ok(()) 
    //};

    if !player.is_dead {
        enemies_check_collisions(
            &mut player, 
            &mut enemies);
        manage_lethal_tile_collision(
            &lethal_tiles, 
            &mut player
        );
        player_move(
            &mut player,
            delta,
            &mut collision_map
        );
        map_move(
            &mut player,
            &mut lethal_tiles,
            &mut collision_map,
            &mut enemies
        );
        manage_player_collision_with_tile(
            &mut(*player), 
            &mut collision_map
        ); 
    }
    
    manage_particles(
        &mut particles,
        delta
    );

    enemies_move(
        &mut enemies,
        delta
    );

    restore_sand_tiles(
        &mut player,
        &mut collision_map
    );

    let res = main_draw(
        &mut collision_map,
        &mut player,
        &mut enemies,
        &mut particles
    );
    res
}

