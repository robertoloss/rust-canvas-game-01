use crate::{collisions::normal_tile_collision::normal_tile_collision, Player, Tile};

pub struct Coin {
    pub tile: Tile,
    pub active: bool,
}
impl Coin {
    pub fn player_collision(&self, player: &Player) -> bool {
        let tile_size = player.tile_size;
        if self.tile.position.x + 20. > player.position.x + tile_size  { return false };
        if self.tile.position.x + tile_size < player.position.x + 20.  { return false };
        if self.tile.position.y > player.position.y + tile_size -20.  { return false };
        if self.tile.position.y + tile_size < player.position.y  { return false };
        true
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    } 
}
