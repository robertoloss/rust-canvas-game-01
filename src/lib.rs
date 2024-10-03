mod map;
mod collisions;
mod player;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement };
use std::{collections::HashMap, sync::Mutex};
use map::*;
use collisions::*;
use player::*;
use web_time::{Instant};


lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
    static ref LAST_TIMESTAMP: Mutex<Option<Instant>> = Mutex::new(None);
}
#[wasm_bindgen]
pub fn movement(key_code: i32) {
    let mut player = PLAYER.lock().unwrap();
    match key_code {
        0 => {
            player.moves.left = true;
            player.moves.right = false;
        },
        1 => {
            player.moves.right = true;
            player.moves.left = false;
        },
        2 => {
            player.moves.jump = true;
        },
        _ => {}
    }
}
#[wasm_bindgen]
pub fn stop_movement(key_code: i32) {
    //console::log_1(&JsValue::from_str(&format!("key_code: {}", key_code)));
    let mut player = PLAYER.lock().unwrap();
    match key_code {
        0 => {
            player.moves.left = false;
        },
        1 => {
            player.moves.right = false;
        },
        2 => {
            player.moves.jump = false;
        },
        _ => {}
    }
}
#[wasm_bindgen]
pub fn initialize() {
    let mut map_collisions = MAP_COLLISIONS.lock().unwrap();
    let player = PLAYER.lock().unwrap();
    *map_collisions = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player));
}

fn get_context(player: &Player) -> Result<(CanvasRenderingContext2d,HtmlCanvasElement), JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("should have a document on window")?;
    let canvas = document.get_element_by_id("gameCanvas").ok_or("can't find gameCanvas")?;
    
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width((player.screen_tiles * (player.tile_size as usize)) as u32);
    canvas.set_height((player.screen_tiles * (player.tile_size as usize)) as u32);
    let context = canvas.get_context("2d")?.ok_or("couldn't get 2D context")?;
    let context: CanvasRenderingContext2d = context.dyn_into::<CanvasRenderingContext2d>()?;

    Ok((context,canvas))
}

fn generate_map_collisions(origin_x: usize, origin_y: usize, player: &Player) -> HashMap<(usize,usize), Tile> {
    let mut collisions_map = HashMap::new(); 
    let game_map = get_map();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    
    if game_map.len() == 0 {
        return collisions_map
    }
    for y in origin_y..origin_y + num_of_tiles {
        for x in origin_x..origin_x + num_of_tiles {
            if game_map[y][x] == 0 {
                let tile = Tile {
                    tile_pos: Vec2usize {
                        x: (x % num_of_tiles),
                        y: (y % num_of_tiles)
                    },
                    position: Vec2 {
                        x: (x % num_of_tiles) as f64 * tile_size,
                        y: (y % num_of_tiles) as f64 * tile_size
                    }
                };
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    tile
                );
            }
        }
    }
    collisions_map
}

#[wasm_bindgen]
pub fn get_delta_time() -> f64 {
    let player = PLAYER.lock().unwrap();
    player.delta_time
}

#[wasm_bindgen]
pub fn render(is_mobile: bool) -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;

    let mut last_timestamp = LAST_TIMESTAMP.lock().unwrap();
    let current_timestamp = Instant::now();

    let delta_time = if let Some(last) = *last_timestamp {
        if !is_mobile { 
            current_timestamp.duration_since(last).as_secs_f64() * 60.
        } else {
            current_timestamp.duration_since(last).as_secs_f64() * 200.0
        }
    } else {
        0.0
    };
    player.delta_time = delta_time;

    *last_timestamp = Some(current_timestamp);

    //console::log_1(&JsValue::from_str(&format!("rust delta: {}", delta_time)));
    
    
    player.velocity.x = if player.moves.right { 
        4.0 * delta_time 
    } else if player.moves.left { 
        -4.0 * delta_time
    } else { 0. };
    //player.velocity.y = if player.moves.down { 4.0 } else if player.moves.up { -4.0 } else { 0. };
    
    if player.moves.jump {
        player.moves.jump = false;
        player.velocity.y = -9. * delta_time;
    }
    if player.velocity.y < 100.0 {
        player.velocity.y += 0.5 * delta_time;
    }

    player.position.x += player.velocity.x * delta_time;
    player.position.y += player.velocity.y * delta_time;

    if player.position.x > tile_size * (num_of_tiles as f64) {
        player.map_origin.x += num_of_tiles;
        player.position.x = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player));
    }
    if player.position.x < -tile_size {
        player.map_origin.x -= num_of_tiles;
        player.position.x = ((num_of_tiles - 1) as f64) * tile_size;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player));
    }
    if player.position.y > tile_size * (num_of_tiles as f64) {
        player.map_origin.y += num_of_tiles;
        player.position.y = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player));
    }
    if player.position.y < -tile_size {
        player.map_origin.y -= num_of_tiles;
        player.position.y = ((num_of_tiles as f64) - 1.0) * tile_size;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player));
    }
    //console::log_1(&JsValue::from_str(&format!("map_origin: {},{}", player.map_origin.x, player.map_origin.y)));

    manage_player_collision_with_tile(&mut(*player), &collision_map);

    let game_map = get_map();
    match get_context(&(*player)) {
        Ok((context, canvas)) => {
            let ctx = &context;
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            for y in player.map_origin.y..player.map_origin.y + num_of_tiles {
                for x in player.map_origin.x..player.map_origin.x + num_of_tiles {
                     if  game_map[y][x] == 0 {
                        ctx.set_fill_style(&JsValue::from_str("gray"));
                        ctx.fill_rect(
                            (x % num_of_tiles) as f64 * tile_size, 
                            (y % num_of_tiles) as f64 * tile_size, 
                            tile_size, 
                            tile_size
                        );
                     }
                }
            }

            ctx.set_fill_style(&JsValue::from_str("#b52c1d"));
            ctx.fill_rect(player.position.x, player.position.y, tile_size, tile_size);
            //ctx.set_image_smoothing_enabled(false);

            ctx.set_font("12px Arial");
            ctx.set_fill_style(&JsValue::from_str("yellow"));
            
            
            let delta_time_text = format!("Delta Time: {:.2}", delta_time);
            ctx.fill_text(&delta_time_text, canvas.width() as f64 - 100.0, 20.0).unwrap();
        },
        Err(e) => eprintln!("Error getting context: {:?}", e)
    }
    Ok(())
}

