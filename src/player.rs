use web_sys::HtmlImageElement;

use crate::{LeftRight, Vec2usize};

#[derive(Debug, Default)]
pub struct ThreadSafeImage(pub Option<wasm_bindgen::JsValue>);
unsafe impl Send for ThreadSafeImage {}
unsafe impl Sync for ThreadSafeImage {}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug)]
pub struct Direction {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64
}
#[derive(Debug)]
pub struct Moves {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
    pub stop_jump: bool,
}
#[derive(Debug)]
pub struct Player {
    pub position: Vec2,
    pub hitbox: Direction,
    pub velocity: Vec2,
    pub gravity: f64,
    pub moves: Moves,
    pub facing_right: bool,
    pub map_origin: Vec2usize,
    pub tile_size: f64,
    pub tile_size_plus_off: f64,
    pub screen_tiles: usize,
    pub can_cling: LeftRight,
    pub wants_to_cling: bool,
    pub is_clinging: bool,
    pub delta: f64,
    pub tile_image: ThreadSafeImage,
    pub player_image: ThreadSafeImage,
    pub player_image_left: ThreadSafeImage,
    pub player_image_cling: ThreadSafeImage,
    pub player_image_cling_left: ThreadSafeImage,
}
impl Default for Player {
    fn default() -> Self {
        let tile_size = 48.0;
        Player {
            position: Vec2 {
                x: 350.0,
                y: 650.0
            },
            hitbox: Direction {
                up: 0.,
                down: 0.,
                left: 6. * 3.0, // pixel * sprite-pixel
                right: 6. * 2.0,
            },
            velocity: Vec2 {
                x: 0.0,
                y: 0.0
            },
            gravity: 0.5,
            moves: Moves {
                left: false,
                right: false,
                up: false,
                down: false,
                jump: false,
                stop_jump: false,
            },
            facing_right: true,
            map_origin: Vec2usize {
                x: 0,
                y: 0
            },
            tile_size,
            tile_size_plus_off: tile_size + 0.0,
            screen_tiles: 16,
            can_cling: LeftRight::None,
            wants_to_cling: false,
            delta: 0.0,
            is_clinging: false,
            tile_image: ThreadSafeImage(None),
            player_image: ThreadSafeImage(None),
            player_image_left: ThreadSafeImage(None),
            player_image_cling: ThreadSafeImage(None),
            player_image_cling_left: ThreadSafeImage(None),
        }
    }
}
