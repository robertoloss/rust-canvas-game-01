use web_sys::HtmlImageElement;
use crate::{SpriteSheet, Vec2, Player};

pub trait EnemyTrait: Send + Sync {
    fn moves(&mut self);
    fn draw(&self);
    fn change_direction(&mut self);
    fn get_direction(&self) -> &LeftRight;
    fn direction_is_left(&self) -> bool;
    fn get_spritesheet(&mut self) -> &mut SpriteSheet;
    fn get_sheetname(&self) -> String;
    fn set_image(&mut self, image: HtmlImageElement);
    fn position(&self) -> Vec2;
    fn check_collision(&self, player: &mut Player);
}
#[derive(PartialEq,Debug)]
pub enum LeftRight {
    Left,
    Right
}


