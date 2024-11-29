use std::{collections::HashMap, u32};
use crate::Vec2usize;
use crate::collisions::types::LeftRight;

#[derive(Clone)]
#[derive(Debug, Default)]
pub struct ThreadSafeImage(pub Option<wasm_bindgen::JsValue>);
unsafe impl Send for ThreadSafeImage {}
unsafe impl Sync for ThreadSafeImage {}

#[derive(Clone,Debug,serde::Serialize)]
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
    pub airborne: bool,
    pub stop_jump: bool,
}
#[derive(Clone,Debug)]
pub struct SpriteSheet {
    pub sheet: ThreadSafeImage,
    pub pointer_y: f64,
    pub tile_position_pointer_y: f64,
    pub counter: u32,
    pub counter_limit: u32,
    pub pointer_y_limit: f64,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TileToRestore {
    pub tile_coordinates: Vec2usize,
    pub counter: usize,
    pub counter_limit: usize,
    pub remove_tile: bool
}
#[derive(Debug)]
pub struct Player {
    pub position: Vec2,
    pub is_dead: bool,
    pub position_spawn: Vec2,
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
    pub clinging_tile_coord: Option<(usize,usize)>,
    pub delta: f64,
    pub images: HashMap<String, ThreadSafeImage>,
    pub sprite_sheets: HashMap<String, SpriteSheet>,
    pub sprite_counter: u32,
    pub tiles_to_restore: Vec<TileToRestore>,
    pub time_to_restore: usize,
    //pub is_on_the_ground: bool,
}
