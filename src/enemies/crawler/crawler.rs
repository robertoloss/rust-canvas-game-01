use web_sys::HtmlImageElement;
use crate::{enemies::types::{EnemyTrait, LeftRight}, Player, SpriteSheet, Vec2};
use crate::ThreadSafeImage;


#[derive(Debug)]
pub struct Crawler {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: LeftRight,
    pub initial_direction: LeftRight,
    pub spritesheet: SpriteSheet,
    pub sheet_name: String,
    pub initial_position: Vec2,
    pub limit_position: Vec2
}

impl EnemyTrait for Crawler {
    fn get_sheetname(&self) -> String {
        self.sheet_name.clone()
    }
    fn set_image(&mut self, image: HtmlImageElement) {
        self.spritesheet.sheet = ThreadSafeImage(Some(image));
    }
    fn check_collision(&self, player: &mut Player) {
        let tile_size = player.tile_size;
        if self.position.x + 20. > player.position.x + tile_size  { return };
        if self.position.x + tile_size < player.position.x + 20.  { return };
        if self.position.y > player.position.y + tile_size -20.  { return };
        if self.position.y + tile_size < player.position.y  { return };
        player.is_dead = true;
    }
    fn direction_is_left(&self) -> bool {
        self.direction == LeftRight::Left
    }
    fn get_direction(&self) -> &LeftRight {
        &self.direction
    }
    fn draw(&self) {
        todo!()
    }
    fn moves(&mut self) {
        let in_dir_left = self.initial_direction == LeftRight::Left;
        let x_not_at_left_limit = self.position.x > 
            if in_dir_left { self.limit_position.x } else { self.initial_position.x };
        let x_not_at_right_limit = self.position.x < 
            if in_dir_left { self.initial_position.x } else { self.limit_position.x };

        if self.direction == LeftRight::Left {
            if x_not_at_left_limit {
                self.position.x += self.velocity.x;
            } else {
                self.change_direction();
            }
        } else {
            if x_not_at_right_limit {
                self.position.x += self.velocity.x;
            } else {
                self.change_direction();
            }
        }
    }
    fn change_direction(&mut self) {
        self.direction = if self.direction == LeftRight::Left { LeftRight::Right } else { LeftRight::Left };
        self.velocity.x = -self.velocity.x;
        self.sheet_name = if self.direction == LeftRight::Left { 
            "crawler_1_0".to_string() 
        } else { 
            "crawler_1_0_R".to_string() 
        };
        self.spritesheet.sheet = ThreadSafeImage(None);
    }
    fn get_spritesheet(&mut self) -> &mut SpriteSheet {
        &mut self.spritesheet
    }
    fn position(&self) -> Vec2 {
        self.position.clone()
    }
}

