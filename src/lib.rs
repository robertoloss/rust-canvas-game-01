use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, console};
use std::sync::Mutex;
mod map;
use map::*;

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
                x: 0.0,
                y: 0.0
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
}

#[wasm_bindgen]
pub fn move_right() {
    let mut player = PLAYER.lock().unwrap();
    player.velocity.x = 2.0; 
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
    //let mut player = PLAYER.lock().unwrap();
    //let game_map = get_map();
    //match get_context() {
    //    Ok((context, _)) => {
    //        let ctx = &context;
    //        game_map
    //            .iter().enumerate().for_each(move |(y, row)| { 
    //                row.iter().enumerate().for_each(move |(x,tile)| {
    //                     if *tile == 0 {
    //                        ctx.set_fill_style(&JsValue::from_str("blue"));
    //                        ctx.fill_rect(x as f64*50.0, y as f64 * 50.0, 50., 50.0);
    //                        console::log_1(&JsValue::from(format!("Filling blue at {}, {}", x, y)));
    //                     }
    //                 })
    //            });
    //    },
    //    Err(e) => eprintln!("Error getting context: {:?}", e)
    //}
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


#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();

    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;

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

