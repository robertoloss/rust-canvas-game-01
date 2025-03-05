use crate::{Tile,Player};

pub fn lethal_tile_collision(tile: &Tile, player: &Player) -> bool {
    let tile_size = player.tile_size;
    if tile.position.x > player.position.x + tile_size  { return false };
    if tile.position.x + tile_size < player.position.x  { return false };
    if tile.position.y > player.position.y + (tile_size / 2.)  { return false };
    if tile.position.y + tile_size < player.position.y  { return false };
    true
}

pub fn manage_lethal_tile_collision(
    lethal_tiles: &Vec<Tile>,
    player: &mut Player
) {
    for tile in lethal_tiles.iter() {
        if lethal_tile_collision(&tile, &player) {
            player.is_dead = true;
        }
    }
}
