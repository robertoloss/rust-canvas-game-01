use crate::{SpriteSheet, Vec2};

pub trait EnemyTrait: Send + Sync {
    fn moves(&mut self);
    fn draw(&self);
    fn change_direction(&mut self);
    fn get_spritesheet(&mut self) -> &mut SpriteSheet;
    fn position(&self) -> Vec2;
}
#[derive(PartialEq)]
pub enum LeftRight {
    Left,
    Right
}

pub struct Crawler {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: LeftRight,
    pub initial_direction: LeftRight,
    pub spritesheet: SpriteSheet,
    pub initial_position: Vec2,
    pub limit_position: Vec2
}

impl EnemyTrait for Crawler {
    fn draw(&self) {
        todo!()
    }
    fn moves(&mut self) {
        let in_dir_left = self.initial_direction == LeftRight::Left;
        if in_dir_left {
            if self.direction == LeftRight::Left {
                if self.position.x > self.limit_position.x {
                    self.position.x += self.velocity.x;
                } else {
                    self.change_direction();
                }
            } else {
                if self.position.x < self.initial_position.x {
                    self.position.x += self.velocity.x;
                } else {
                    self.change_direction();
                }
            }
        } else {
            if self.direction == LeftRight::Left {
                if self.position.x > self.initial_position.x {
                    self.position.x += self.velocity.x;
                } else {
                    self.change_direction();
                }
            } else {
                if self.position.x < self.limit_position.x {
                    self.position.x += self.velocity.x;
                } else {
                    self.change_direction();
                }
            }
        }
    }
    fn change_direction(&mut self) {
        self.direction = if self.direction == LeftRight::Left { LeftRight::Right } else { LeftRight::Left };
        self.velocity.x = -self.velocity.x;
    }
    fn get_spritesheet(&mut self) -> &mut SpriteSheet {
        &mut self.spritesheet
    }
    fn position(&self) -> Vec2 {
        self.position.clone()
    }
}
