use crate::{enemies::types::{EnemyTrait, UpDown}, Player, SpriteSheet, Vec2};
use crate::ThreadSafeImage;

#[derive(Debug)]
pub struct Climber {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: UpDown,
    pub initial_direction: UpDown,
    pub spritesheet: SpriteSheet,
    pub sheet_name: String,
    pub initial_position: Vec2,
    pub limit_position: Vec2,
    pub facing_right: bool
}

impl EnemyTrait for Climber {
    fn get_sheetname(&self) -> String {
        self.sheet_name.clone()
    }

    fn check_collision(&self, player: &mut Player) {
        let tile_size = player.tile_size;

        let offset_left = if self.facing_right { 0. } else { 24. };
        let offset_right = if self.facing_right { 24. } else { 0. };

        if self.position.x + offset_left > player.position.x + tile_size  { return };
        if self.position.x + tile_size < player.position.x + offset_right { return };
        if self.position.y + 16. > player.position.y + tile_size  { return };
        if self.position.y + tile_size < player.position.y + 16.  { return };

        player.is_dead = true;
    }

    fn moves(&mut self) {
        let in_dir_up = self.initial_direction == UpDown::Up;
        let y_not_at_up_limit = self.position.y > 
            if in_dir_up { self.limit_position.y } else { self.initial_position.y };
        let y_not_at_down_limit = self.position.y < 
            if in_dir_up { self.initial_position.y } else { self.limit_position.y };

        if self.direction == UpDown::Up {
            if y_not_at_up_limit {
                self.position.y += self.velocity.y;
            } else {
                self.change_direction();
            }
        } else {
            if y_not_at_down_limit {
                self.position.y += self.velocity.y;
            } else {
                self.change_direction();
            }
        }
    }
    fn change_direction(&mut self) {
        self.direction = if self.direction == UpDown::Up { UpDown::Down } else { UpDown::Up };
        self.velocity.y = -self.velocity.y;
        self.sheet_name = if self.facing_right { "climber_R".to_string() } else { "climber_L".to_string() }; 
        self.spritesheet.sheet = ThreadSafeImage(None);
    }
    fn get_spritesheet(&mut self) -> &mut SpriteSheet {
        &mut self.spritesheet
    }
    fn position(&self) -> Vec2 {
        self.position.clone()
    }
}

