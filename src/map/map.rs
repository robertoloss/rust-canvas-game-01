use crate::player::types::Vec2;
use crate::SpriteSheet;

#[derive(Clone,Debug,serde::Serialize,PartialEq)]
pub struct Vec2usize {
    pub x: usize,
    pub y: usize
}

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct Tile {
    pub map_origin: Vec2usize,
    pub tile_pos: Vec2usize,
    pub position: Vec2,
    pub sheet: Option<SpriteSheet>,
    pub touched_by_player: bool,
    pub just_restored: bool,
    pub hanging_tile: bool,
    pub spawning_tile: bool
}

pub fn get_map() -> Vec<Vec<u8>> {
    let w: u8 = 0;
    let c: u8 = 12; 
    let s: u8 = 20;

    vec![
          // 1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8
        vec![w,7,7,7,7,7,7,7,7,7,7,7,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,c,1,1,2,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,w,w,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,w,w,w,w,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,w,w,w,w,1,1,1,1,1,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,7,7,7,1,1,1,1,1,w,1,1,w,w,1,1,1,1,1,1,1,w,w,w,w,w,1,1,w],
        vec![w,1,1,1,1,1,1,w,9,9,w,1,1,1,4,w,w,1,c,1,1,1,1,1,1,1,1,w,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,7,7,7,7,1,1,1,1,w,w,w,w,1,1,1,1,c,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,w,1,1,1,1,1,1,1,w,w,1,1,w,w,w,w,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,w,1,1,1,1,1,1,1,w,w,1,1,w,1,1,w,1,1,1,1,1,1,1,1,w],
        vec![w,3,1,1,w,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,c,1,w,1,1,1,1,w,w,w,w,w,1,w,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,w,1,1,c,1,1,1,1,1,1,w,w,1,1,1,6,6,6,w,w,w,1,1,7,7,7,w,w,7,7,7,1,1,1,1,1,1,1,1,1,7,7,w],
        vec![w,1,1,1,w,w,6,6,6,1,1,1,1,c,1,1,1,1,1,1,1,1,1,1,1,w,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,1,1,1,1,1,1,1,c,w,1,1,1,1,1,1,1,1,1,1,1,7,7,1,1,1,1,1,1,1,1,1,1,1,6,6,6,6,6,6,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,c,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,2,1,1,1,1,w,w,w,1,1,1,1,1,1,2,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,w,w,w,s,9,9,w,w,w,w,w,w,s,w,w,w,9,9,w,w,w,w,w,1,1,1,w,w,w,w,w,w,w,w,9,9,9,w,w,1,1,1,w],
        vec![w,1,1,1,w,w,w,w,w,7,7,7,7,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,1,1,1,w,w,w,w,w,7,7,7,w,w,w,w,w,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,6,6,6,1,1,1,1,1,1,1,2,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,w,w,1,1,1,1,7,7,7,1,w,w,w,w,w,w,w,1,1,1,1,1,1,1,1,w,w,w,w,w,w,w,1,1,1,1,1,1,1,1,7,7,7,7,w],
        vec![w,1,1,1,w,w,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,1,1,w,w,1,1,1,1,1,1,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,9,9,w,1,1,1,1,1,1,1,1,w,w,w,w,w,1,1,1,1,w,w,w,1,1,1,1,1,w,w,w,w,1,1,1,1,1,w,w,w,w,1,1,1,1,1,w],
        vec![w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,w,w,w,w,w,1,1,1,1,w,w,1,1,1,1,1,w,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,w,6,6,w,1,1,w,w,1,w,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,w,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,w,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,w,w,w,w,1,1,w,w,w,w,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,w,w,w,1,1,1,1,1,w,w,w,w,1,1,w,w,w,1,1,w,w,w,w,w,w,w,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w],
        vec![w,1,1,1,1,w,w,w,1,1,1,1,w,w,1,1,1,w,1,1,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,w,w,w,w,w,w,w,w,w,1,1,1,w,w,w,w,w,w,w,w,w,w,1,1,1,w,w,w,w,w,w,w,w,w,w,w,w,w,1,1,1,w,w,w,w,w,w],
        vec![w,7,7,7,7,7,7,7,7,7,1,1,1,w,w,w,w,w,w,w,7,7,7,1,1,1,w,w,w,w,w,w,w,w,w,w,w,w,w,1,1,1,w,w,w,w,w,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,6,6,6,1,1,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,6,6,6,1,1,w,w,1,w,1,1,1,1,1,1,w,w,w,1,1,1,w,w,1,1,1,1,w,w,w,w,w,w,1,1,w,w,w],
        vec![w,w,w,w,6,6,6,6,w,w,9,9,w,1,1,w,w,1,1,w,1,1,1,1,1,1,1,1,w,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,w,w,w,w,1,1,w,w,1,1,1,w,1,1,1,1,1,1,1,1,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,w,1,1,1,1,1,1,1,1,1,w,w,w,w,1,1,1,w,w,w,1,1,1,1,1,w,w],
        vec![w,1,1,1,w,6,6,w,w,1,1,1,1,w,w,w,w,1,1,1,1,1,w,w,w,w,w,1,1,1,1,w,w,w,w,1,1,1,1,w,1,1,1,1,1,w,w,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,w,1,1,1,1,w,w,w,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,w,w,1,1,1,1,1,1,1,1,w,w,w,w,1,1,1,1,1,w,w,w,1,1,w,w,w,w,w],
        vec![w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,w,1,1,1,1,w,w,1,1,1,1,1,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,w,w,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,w,9,9,w,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w,w,1,1,1,1,1,1,1,1,w,w,w,w],
        vec![w,1,1,1,w,w,w,w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,w],
        vec![w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w,w],
    ]
}
