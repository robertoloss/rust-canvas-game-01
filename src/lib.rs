mod map;
mod particles;
mod utils;
mod collisions;
mod player;
mod enemies;
mod coins;
mod draw;
use coins::manage_coins::manage_coins;
use enemies::enemies_check_collision::enemies_check_collisions;
use enemies::enemies_move::enemies_move;
use enemies::types::EnemyTrait;
use map::generate_persisting_entities::generate_persisting_entities;
use map::restore_sand_tiles::restore_sand_tiles;
use particles::lava_particles::lava_particles;
use particles::manage_particles::manage_particles;
use particles::spawn_particles::spawn_particles;
use particles::types::Particle;
use particles::wind_particles::wind_particles;
use utils::extern_c::is_game_paused;
use utils::extern_c::screen_size;
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
use crate::coins::types::Coin;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
    static ref LETHAL_TILES: Mutex<Vec<Tile>> = Mutex::new(vec![]);
    static ref ENEMIES: Mutex<Vec<Box<dyn EnemyTrait>>> = Mutex::new(vec![]);
    static ref PARTICLES: Mutex<Vec<Particle>> = Mutex::new(vec![]);
    static ref LAVA_TILES: Mutex<Vec<Tile>> = Mutex::new(vec![]);
    static ref COINS: Mutex<Vec<Coin>> = Mutex::new(vec![]);
}


#[wasm_bindgen]
pub fn initialize() {
    set_panic_hook();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let mut coins = COINS.lock().unwrap();
    let mut lava_tiles = LAVA_TILES.lock().unwrap();
    let player = PLAYER.lock().unwrap();

    (*collision_map,*lethal_tiles) = generate_map_collisions(
        player.map_origin.x, 
        player.map_origin.y, 
        &player, 
        &mut enemies,
        true,
        &mut lava_tiles
    );
    generate_persisting_entities(
        &mut coins, 
        &player
    );
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let mut lethal_tiles = LETHAL_TILES.lock().unwrap();
    let mut enemies = ENEMIES.lock().unwrap();
    let mut particles = PARTICLES.lock().unwrap();
    let mut coins = COINS.lock().unwrap();
    let mut lava_tiles = LAVA_TILES.lock().unwrap();
    
    let mut delta = (player.delta / 60.).clamp(0.9, 1.1);
    if screen_size() < 800 { 
        delta = delta.clamp(2.4, 2.6) 
    };

    if !all_images_present(&player.images) { return Ok(()) }
    if !all_sprite_sheets_present(&player.sprite_sheets) { return Ok(()) }
    if delta == 0. { return Ok(()) }
    if is_game_paused() { return Ok(()) }

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
            &mut particles,
            &mut player,
            &mut lethal_tiles,
            &mut collision_map,
            &mut enemies,
            &mut lava_tiles
        );
        manage_player_collision_with_tile(
            &mut(*player), 
            &mut collision_map,
            &mut particles
        ); 
    }

    if player.map_origin == player.map_origin_spawn {
        spawn_particles(
            &player,
            &mut particles
        );
    }

    //wind_particles(&mut particles);

    lava_tiles
        .iter()
        .for_each(|tile| {
            lava_particles(
                &mut particles, 
                tile.position.clone()
            );
        });

    manage_particles(
        &mut particles,
        delta
    );

    manage_coins(
        &mut player,
        &mut coins,
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
        &mut lethal_tiles,
        &mut collision_map,
        &mut player,
        &mut enemies,
        &mut particles,
        &mut coins,
        &mut lava_tiles
    );
    res
}

