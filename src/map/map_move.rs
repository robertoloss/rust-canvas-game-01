use std::collections::HashMap;

use web_sys::console;

use crate::enemies::types::EnemyTrait;
use crate::{enemies, generate_map_collisions};
use crate::{Tile, Player};

pub fn map_move(
    player: &mut Player, 
    lethal_tiles: &mut Vec<Tile>,
    num_of_tiles: usize,
    tile_size: f64,
    collision_map: &mut HashMap<(usize, usize), Tile>,
    enemies: &mut Vec<Box< dyn EnemyTrait>>
) {
    if player.position.x > tile_size * (num_of_tiles as f64) {
        player.map_origin.x += num_of_tiles;
        player.position.x = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).0;
        *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).1;
    }
    if player.position.x < -tile_size {
        player.map_origin.x -= num_of_tiles;
        player.position.x = ((num_of_tiles - 1) as f64) * tile_size;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).0;
        *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).1;
    }
    if player.position.y > tile_size * (num_of_tiles as f64) {
        player.map_origin.y += num_of_tiles;
        player.position.y = 0.;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).0;
        *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).1;
    }
    if player.position.y < -tile_size {
        player.map_origin.y -= num_of_tiles;
        player.position.y = ((num_of_tiles as f64) - 1.0) * tile_size;
        *collision_map = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).0;
        *lethal_tiles = generate_map_collisions(player.map_origin.x, player.map_origin.y, &(*player), enemies).1;
    }
}
