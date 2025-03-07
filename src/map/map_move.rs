use std::collections::HashMap;
use crate::coins::types::Coin;
use crate::enemies::types::EnemyTrait;
use crate::generate_map_collisions;
use crate::{Tile, Player};

fn update_collisions_and_lethal_tiles(
    player: &mut Player, 
    lethal_tiles: &mut Vec<Tile>,
    collision_map: &mut HashMap<(usize, usize), Tile>,
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
    coins: &mut Vec<Coin>
) {
    (*collision_map,*lethal_tiles) = generate_map_collisions(
        player.map_origin.x, 
        player.map_origin.y, 
        &(*player), 
        enemies,
        coins,
        true
    );
}
pub fn map_move(
    player: &mut Player, 
    lethal_tiles: &mut Vec<Tile>,
    collision_map: &mut HashMap<(usize, usize), Tile>,
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
    coins: &mut Vec<Coin>
) {
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    let buffer = 16.;
    let mut update = false;
    if player.position.x > (tile_size * (num_of_tiles as f64) - buffer) {
        player.map_origin.x += num_of_tiles;
        player.position.x = 0.;
        update = true;
    }
    if player.position.x < -tile_size + buffer {
        player.map_origin.x -= num_of_tiles;
        player.position.x = ((num_of_tiles - 1) as f64) * tile_size;
        update = true;
    }
    if player.position.y > (tile_size * (num_of_tiles as f64)) - buffer {
        player.map_origin.y += num_of_tiles;
        player.position.y = 0.;
        update = true;
    }
    if player.position.y < -tile_size + buffer {
        player.map_origin.y -= num_of_tiles;
        player.position.y = ((num_of_tiles as f64) - 1.0) * tile_size;
        update = true;
    }
    if update {
        update_collisions_and_lethal_tiles(
            player, 
            lethal_tiles, 
            collision_map, 
            enemies, 
            coins
        );
    }
}
