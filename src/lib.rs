use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::sync::Mutex;

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
}

#[wasm_bindgen]
pub fn update() {
    let _player = PLAYER.lock().unwrap();
    // Update player state or game logic here
    //player.pos_y += 1.0;
}


#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let mut player = PLAYER.lock().unwrap();

    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;



    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("gameCanvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.set_fill_style(&JsValue::from_str("red"));
    context.fill_rect(player.position.x, player.position.y, 50.0, 50.0);

    Ok(())
}

