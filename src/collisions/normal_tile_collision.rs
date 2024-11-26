use crate::{Tile,Player};

pub fn normal_tile_collision(tile: &Tile, player: &Player) -> bool {
    let tile_size = player.tile_size;
    if tile.position.x > player.position.x + tile_size  { return false };
    if tile.position.x + tile_size < player.position.x  { return false };
    if tile.position.y > player.position.y + (tile_size / 2.)  { return false };
    if tile.position.y + tile_size < player.position.y  { return false };
    true
}
