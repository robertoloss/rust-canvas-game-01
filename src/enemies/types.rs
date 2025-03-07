use crate::{SpriteSheet, Vec2, Player};

pub trait EnemyTrait: Send + Sync {
    fn moves(&mut self, delta: f64);
    fn change_direction(&mut self);
    fn get_spritesheet(&mut self) -> &mut SpriteSheet;
    fn get_sheetname(&self) -> String;
    fn position(&self) -> Vec2;
    fn check_collision(&self, player: &mut Player);
}

#[derive(PartialEq,Debug)]
pub enum LeftRight {
    Left,
    Right
}

#[derive(PartialEq,Debug)]
pub enum UpDown {
    Up,
    Down
}


