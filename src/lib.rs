use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, console};
use std::{collections::HashMap, sync::Mutex};
mod map;
use map::*;

#[derive(Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

struct Player {
    position: Vec2,
    velocity: Vec2,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            position: Vec2 {
                x: 50.0,
                y: 50.0
            },
            velocity: Vec2 {
                x: 0.0,
                y: 0.0
            }
        }
    }
}

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn move_right() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.x = 2.0; 
}
#[wasm_bindgen]
pub fn stop_horizontal() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.x = 0.0; 
}
#[wasm_bindgen]
pub fn stop_vertical() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.y = 0.0; 
}


#[wasm_bindgen]
pub fn move_left() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.x = -2.0; 
}

#[wasm_bindgen]
pub fn move_up() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.y = -2.0; 
}

#[wasm_bindgen]
pub fn move_down() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.y = 2.0; 
}

#[wasm_bindgen]
pub fn initialize() {
    let mut map_collisions = MAP_COLLISIONS.lock().unwrap();
    *map_collisions = generate_map_collisions();
    //let map_string = format!("{:?}", *map_collisions);
    //console::log_1(&JsValue::from_str(&map_string));
}

#[wasm_bindgen]
pub fn update() {
    let _player = PLAYER.lock().unwrap();
    // Update player state or game logic here
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

fn generate_map_collisions() -> HashMap<(usize,usize), Tile> {
    let mut collisions_map = HashMap::new(); 
    let game_map = get_map();
    
    if game_map.len() == 0 {
        return collisions_map
    }
    for y in 0..game_map.len() {
        for x in 0..game_map[0].len() {
            if game_map[y][x] == 0 {
                let tile = Tile {
                    tile_pos: Vec2usize {
                        x,
                        y
                    },
                    position: Vec2 {
                        x: x as f64 * 50.0,
                        y: y as f64 * 50.0
                    }
                };
                collisions_map.insert((x,y), tile);
            }
        }
    }
    collisions_map
}

fn tile_collision(tuple: (usize, usize), collision_map: &HashMap<(usize, usize), Tile>) -> (bool,Option<&map::Tile>) {
    (collision_map.contains_key(&tuple),collision_map.get(&tuple))
}

fn manage_player_collision_with_tile(player: &mut Player, collision_map: &HashMap<(usize, usize), Tile>) {
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
                let tile = tile_collision(top_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x - 50.0;
                }
            } else {
                let tile = tile_collision(bottom_right, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x - 50.0;
                }

            }
        } else if tile_collision(top_left, &collision_map).0 || tile_collision(bottom_left, &collision_map).0  {
            player.velocity.x = 0.;
            if tile_collision(top_left, &collision_map).0 {
                let tile = tile_collision(top_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x + 50.0;
                }
            } else {
                let tile = tile_collision(bottom_left, &collision_map).1;
                if let Some(t) = tile {
                    player.position.x = t.position.x + 50.0;
                }

            }
            
        }
    }
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let collision_map = MAP_COLLISIONS.lock().unwrap();

    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;

    manage_player_collision_with_tile(&mut(*player), &collision_map);

    let game_map = get_map();
    match get_context() {
        Ok((context, canvas)) => {
            let ctx = &context;
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            game_map
                .iter().enumerate().for_each(move |(y, row)| { 
                    if y > 15 { return }
                    row.iter().enumerate().for_each(move |(x,tile)| {
                        if x > 15 { return }
                         if *tile == 0 {
                            ctx.set_fill_style(&JsValue::from_str("blue"));
                            ctx.fill_rect(x as f64*50.0, y as f64 * 50.0, 50., 50.0);
                            //console::log_1(&JsValue::from(format!("Filling blue at {}, {}", x, y)));
                         }
                     })
                });
            context.set_fill_style(&JsValue::from_str("red"));
            context.fill_rect(player.position.x, player.position.y, 50.0, 50.0);
        },
        Err(e) => eprintln!("Error getting context: {:?}", e)
    }

    Ok(())
}

