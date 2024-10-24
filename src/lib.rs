mod map;
mod collisions;
mod player;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement };
use std::{collections::HashMap, sync::Mutex};
use map::*;
use collisions::*;
use player::*;


lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::default());
    static ref MAP_COLLISIONS: Mutex<HashMap<(usize,usize), Tile>> = Mutex::new(HashMap::new());
}
#[wasm_bindgen]
pub fn movement(key_code: i32) {
    let mut player = PLAYER.lock().unwrap();
    match key_code {
        0 => {
            player.moves.left = true;
            player.moves.right = false;
            if !player.is_clinging {
                player.facing_right = false;
            }
        },
        1 => {
            player.moves.right = true;
            player.moves.left = false;
            if !player.is_clinging {
                player.facing_right = true;
            }
        },
        2 => {
            player.moves.jump = true;
            player.wants_to_cling = false;
            if player.is_clinging {
                player.is_clinging = false;
                if player.moves.right || player.moves.left {
                    player.facing_right = if player.moves.right {
                        true
                    } else {
                        false
                    }
                }
            }
        },
        3 => {
            player.wants_to_cling = true;
        }
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
            player.moves.stop_jump = true;
        },
        3 => {
            player.wants_to_cling = false;
            player.is_clinging = false;
        }
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
pub fn get_and_give_f64(num: Option<f64>) {
    //console::log_1(&JsValue::from_str(&format!("{}", num)));
    let mut player = PLAYER.lock().unwrap();
    match num {
        Some(n) => player.delta = n,
        None => player.delta = 60.
    }
}
#[wasm_bindgen]
pub fn set_tile_image(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.tile_image = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_left = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_cling(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_cling = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_cling_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_cling_left = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_sheet_run_right(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.run_right.sheet = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_sheet_run_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.run_left.sheet = ThreadSafeImage(img.map(|i| i.into()));
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();
    let mut collision_map = MAP_COLLISIONS.lock().unwrap();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;

    let delta = player.delta / 60.; //0.016 * (0.016 * 1000. * 3.3);
    //console::log_1( &JsValue::from_str( &format!( "delta {}", player.delta) ));
    if delta == 0. {
        return Ok(())
    }
    player.velocity.x = if player.moves.right { 
        player.horizontal_velocity 
    } else if player.moves.left { 
        -player.horizontal_velocity
    } else { 0. };
    //player.velocity.y = if player.moves.down { 4.0 } else if player.moves.up { -4.0 } else { 0. };

    if !player.is_clinging {
        if player.velocity.x > 0. {
            player.facing_right = true
        }
        if player.velocity.x < 0. {
            player.facing_right = false
        }
    }
    
    if player.moves.jump {
        player.moves.jump = false;
        player.velocity.y = player.jump_velocity; //-10.1
    }
    if player.moves.stop_jump {
        player.moves.stop_jump = false;
        if player.velocity.y < -3. {
            player.velocity.y += 3.//3.
        }
    }
    if player.velocity.y < player.max_fall_velocity {
        player.velocity.y += player.gravity / delta
    }
    if player.wants_to_cling && player.can_cling != collisions::LeftRight::None {
        player.is_clinging = true
    }
    if player.is_clinging {
        player.velocity.y = 0.;
        player.velocity.x = 0.;
    }

    player.position.x += player.velocity.x / delta;
    player.position.y += player.velocity.y / delta;

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


    if let Some(js_val) = &player.tile_image.0 {
        let image: HtmlImageElement = js_val.clone().into();
        let game_map = get_map();
        match get_context(&(*player)) {
            Ok((context, canvas)) => {
                let ctx = &context;
                ctx.set_image_smoothing_enabled(false);
                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                ctx.set_fill_style(&JsValue::from_str("black"));
                ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                for y in player.map_origin.y..player.map_origin.y + num_of_tiles {
                    for x in player.map_origin.x..player.map_origin.x + num_of_tiles {
                         if  game_map[y][x] == 0 {
                             ctx.draw_image_with_html_image_element_and_dw_and_dh(
                                &image,
                                (x % num_of_tiles) as f64 * tile_size, 
                                (y % num_of_tiles) as f64 * tile_size, 
                                tile_size,
                                tile_size,
                            )?;
                         }
                    }
                }
                //ctx.set_font("14px Arial, sans-serif");
                //ctx.set_fill_style(&JsValue::from_str("yellow"));
                //let _ = ctx.fill_text(&player.delta.to_string(), 30., 15.);
                //let _ = ctx.fill_text(&delta.to_string(), 30., 30.);
                if player.is_clinging || (player.velocity.x == 0. || player.velocity.y != 0.) {
                    player.sprite_counter = 0;
                }

                let mut _image: &ThreadSafeImage = &ThreadSafeImage(None); 
                player.sprite_counter += 1;

                match player.facing_right {
                    true => _image =  if player.is_clinging {
                        &player.player_image_cling
                    } else if player.velocity.x == 0. || player.velocity.y != 0. {//> 1. || player.velocity.y < -1. { 
                        &player.player_image
                    } else {
                        &player.run_right.sheet
                    },
                    false => _image = if player.is_clinging {
                        &player.player_image_cling_left
                    } else if player.velocity.x == 0. || player.velocity.y != 0. {
                        &player.player_image_left
                    } else {
                        &player.run_left.sheet
                    }
                }

                let mut pointer_y = 0.;
                //console::log_1(&JsValue::from_str(&format!("{}", player.sprite_counter)));

                let player_sprite = _image.0.clone().unwrap().into();
                let is_run_right_sheet = std::ptr::eq(_image, &player.run_right.sheet);
                let is_run_left_sheet = std::ptr::eq(_image, &player.run_left.sheet);

                if is_run_right_sheet {
                    if player.sprite_counter >= player.run_right.counter_limit {
                        player.sprite_counter = 0;
                        player.run_right.pointer_y += tile_size;
                        if player.run_right.pointer_y >= player.run_right.pointer_y_limit {
                            player.run_right.pointer_y = 0.
                        }
                    }
                    pointer_y = player.run_right.pointer_y;
                } else if is_run_left_sheet {
                    if player.sprite_counter >= player.run_left.counter_limit {
                        player.sprite_counter = 0;
                        player.run_left.pointer_y += tile_size;
                        if player.run_left.pointer_y >= player.run_left.pointer_y_limit {
                            player.run_left.pointer_y = 0.
                        }
                    }
                    pointer_y = player.run_left.pointer_y;
                }


                ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &player_sprite,
                    0., pointer_y, tile_size, tile_size,
                    player.position.x, player.position.y, tile_size, tile_size,
                )?;
            },
            Err(e) => eprintln!("Error getting context: {:?}", e)
        }
    }
    Ok(())
}

