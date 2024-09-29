mod map;
mod collisions;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d };
use std::{collections::HashMap, sync::Mutex};
use map::*;
use collisions::*;

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}
struct Moves {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    jump: bool
}
struct Player {
    position: Vec2,
    velocity: Vec2,
    moves: Moves,
    map_origin: Vec2usize
}
impl Default for Player {
    fn default() -> Self {
        Player {
            position: Vec2 {
                x: 350.0,
                y: 650.0
            },
            velocity: Vec2 {
                x: 0.0,
                y: 0.0
            },
            moves: Moves {
                left: false,
                right: false,
                up: false,
                down: false,
                jump: false
            },
            map_origin: Vec2usize {
                x: 0,
                y: 0
            }
        }
    }
}

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn stop_up() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.up = false
}
#[wasm_bindgen]
pub fn stop_down() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.down = false
}
#[wasm_bindgen]
pub fn stop_left() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.left = false
}
#[wasm_bindgen]
pub fn stop_right() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.right = false
}
#[wasm_bindgen]
pub fn stop_jumping() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.jump = false
}

#[wasm_bindgen]
pub fn move_right() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.right = true;
    player.moves.left = false;
}
#[wasm_bindgen]
pub fn move_left() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.left = true;
    player.moves.right = false;
}

#[wasm_bindgen]
pub fn move_up() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.up = true;
    player.moves.down = false;
}

#[wasm_bindgen]
pub fn move_down() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.down = true;
    player.moves.up = false;
}
#[wasm_bindgen]
pub fn jump() {
    let mut player = PLAYER.lock().unwrap();
    player.moves.jump = true;
}

#[wasm_bindgen]
pub fn initialize() {
    let mut map_collisions = MAP_COLLISIONS.lock().unwrap();
    let player = PLAYER.lock().unwrap();
    *map_collisions = generate_map_collisions(player.map_origin.x, player.map_origin.y);
    //let map_string = format!("{:?}", *map_collisions);
    //console::log_1(&JsValue::from_str(&map_string));
}

#[wasm_bindgen]
pub fn update() {
    let _player = PLAYER.lock().unwrap();
    //player.pos_y += 1.0;
}

fn get_context() -> Result<(CanvasRenderingContext2d,HtmlCanvasElement), JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("should have a document on window")?;
    let canvas = document.get_element_by_id("gameCanvas").ok_or("can't find gameCanvas")?;
    
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas.get_context("2d")?.ok_or("couldn't get 2D context")?;
    let context: CanvasRenderingContext2d = context.dyn_into::<CanvasRenderingContext2d>()?;

    Ok((context,canvas))
}

fn generate_map_collisions(origin_x: usize, origin_y: usize) -> HashMap<(usize,usize), Tile> {
    let mut collisions_map = HashMap::new(); 
    let game_map = get_map();
    
    if game_map.len() == 0 {
        return collisions_map
    }
    for y in origin_y..origin_y+16 {
        for x in origin_x..origin_x+16 {
            if game_map[y][x] == 0 {
                let tile = Tile {
                    tile_pos: Vec2usize {
                        x: (x % 16),
                        y: (y % 16)
                    },
                    position: Vec2 {
                        x: (x % 16) as f64 * 50.0,
                        y: (y % 16) as f64 * 50.0
                    }
                };
                collisions_map.insert(
                    ( (x % 16) , (y % 16) ), 
                    tile
                );
            }
        }
    }
    collisions_map
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    
    player.velocity.x = if player.moves.right { 4.0 } else if player.moves.left { -4.0 } else { 0. };
    //player.velocity.y = if player.moves.down { 4.0 } else if player.moves.up { -4.0 } else { 0. };
    
    if player.moves.jump {
        player.moves.jump = false;
        player.velocity.y = -9.;
    }
    if player.velocity.y < 100.0 {
        player.velocity.y += 0.5;
    }

    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;

    if player.position.x > 50.0 * 16.0 {
        player.map_origin.x += 16;
        player.position.x = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y);
    }
    if player.position.x < -50.0 {
        player.map_origin.x -= 16;
        player.position.x = 15.0 * 50.0;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y);
    }
    if player.position.y > 50.0 * 16.0 {
        player.map_origin.y += 16;
        player.position.y = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y);
    }
    if player.position.y < -50.0 {
        player.map_origin.y -= 16;
        player.position.y = 15.0 * 50.0;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y);
    }
    //console::log_1(&JsValue::from_str(&format!("map_origin: {},{}", player.map_origin.x, player.map_origin.y)));

    manage_player_collision_with_tile(&mut(*player), &collision_map);

    let game_map = get_map();
    match get_context() {
        Ok((context, canvas)) => {
            let ctx = &context;
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            for y in player.map_origin.y..player.map_origin.y+16 {
                for x in player.map_origin.x..player.map_origin.x+16 {
                    //let message = format!("tile: {}, {}, {}", game_map[y][x], x, y);
                    //console::log_1(&JsValue::from_str(&message));
                     if  game_map[y][x] == 0 {
                        ctx.set_fill_style(&JsValue::from_str("gray"));
                        ctx.fill_rect(
                            (x % 16) as f64*50.0, 
                            (y % 16) as f64 * 50.0, 
                            50., 
                            50.
                        );
                     }
                }
            }

            context.set_fill_style(&JsValue::from_str("#b52c1d"));
            context.fill_rect(player.position.x, player.position.y, 50.0, 50.0);
        },
        Err(e) => eprintln!("Error getting context: {:?}", e)
    }
    Ok(())
}

