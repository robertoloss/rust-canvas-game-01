use std::u32;

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
    pub left: f64,
    pub right: f64
}
#[derive(Debug)]
pub struct Moves {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub stop_jump: bool,
}
#[derive(Debug)]
pub struct SpriteSheet {
    pub sheet: ThreadSafeImage,
    pub pointer_y: f64,
    pub counter_limit: u32,
    pub pointer_y_limit: f64,
}
#[derive(Debug)]
pub struct Player {
    pub position: Vec2,
    pub hitbox: Direction,
    pub velocity: Vec2,
    pub gravity: f64,
    pub jump_velocity: f64,
    pub max_fall_velocity: f64,
    pub horizontal_velocity: f64,
    pub moves: Moves,
    pub facing_right: bool,
    pub map_origin: Vec2usize,
    pub tile_size: f64,
    pub screen_tiles: usize,
    pub can_cling: LeftRight,
    pub wants_to_cling: bool,
    pub is_clinging: bool,
    pub delta: f64,
    pub tile_image: ThreadSafeImage,
    pub lava_sheet: SpriteSheet,
    pub player_image: ThreadSafeImage,
    pub player_image_left: ThreadSafeImage,
    pub player_image_cling: ThreadSafeImage,
    pub player_image_cling_left: ThreadSafeImage,
    pub run_right: SpriteSheet,
    pub run_left: SpriteSheet,
    pub sprite_counter: u32,
    pub is_on_the_ground: bool,
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
                left: 6. * 2.0, // pixel * sprite-pixel
                right: 6. * 2.0,
            },
            velocity: Vec2 {
                x: 0.0,
                y: 0.0
            },
            gravity: 0.3, //0.5,
            jump_velocity: -8.2, //-10.1,
            horizontal_velocity: 2.3,
            max_fall_velocity: 50.,
            is_on_the_ground: true,
            moves: Moves {
                left: false,
                right: false,
                jump: false,
                stop_jump: false,
            },
            facing_right: true,
            map_origin: Vec2usize {
                x: 0,
                y: 0
            },
            tile_size,
            screen_tiles: 16,
            can_cling: LeftRight::None,
            wants_to_cling: false,
            delta: 0.0,
            is_clinging: false,
            tile_image: ThreadSafeImage(None),
            lava_sheet: SpriteSheet {
                sheet: ThreadSafeImage(None),
                pointer_y: 0.,
                counter_limit: 4,
                pointer_y_limit: 6. * tile_size,
            },
            player_image: ThreadSafeImage(None),
            player_image_left: ThreadSafeImage(None),
            player_image_cling: ThreadSafeImage(None),
            player_image_cling_left: ThreadSafeImage(None),
            sprite_counter: 0,
            run_right: SpriteSheet {
                sheet: ThreadSafeImage(None),
                pointer_y: 0.,
                counter_limit: 4,
                pointer_y_limit: 8. * 48.,
            },
            run_left: SpriteSheet {
                sheet: ThreadSafeImage(None),
                pointer_y: 0.,
                counter_limit: 4,
                pointer_y_limit: 8. * tile_size,
            }

        }
    }
}
