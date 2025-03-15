use crate::{Player, Tile};

pub fn manage_spawn_collision(
    player: &mut Player,
    tile: &mut Tile
) {
    let tile_is_spawn = tile.spawning_tile;
    if tile_is_spawn {
        player.position_spawn = tile.position.clone();
        player.position_spawn.y -= 48.0;
        player.map_origin_spawn = tile.map_origin.clone();
    }
}
