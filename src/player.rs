use crate::Vec2usize;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
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
    pub velocity: Vec2,
    pub gravity: f64,
    pub moves: Moves,
    pub map_origin: Vec2usize,
    pub tile_size: f64,
    pub tile_size_plus_off: f64,
    pub screen_tiles: usize,
}
impl Default for Player {
    fn default() -> Self {
        let tile_size = 48.0;
        Player {
            position: Vec2 {
                x: 350.0,
                y: 650.0
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
            map_origin: Vec2usize {
                x: 0,
                y: 0
            },
            tile_size,
            tile_size_plus_off: tile_size + 0.1,
            screen_tiles: 16,
        }
    }
}
