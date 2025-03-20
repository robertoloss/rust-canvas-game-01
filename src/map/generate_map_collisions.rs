use std::collections::HashMap;
use crate::log_out_f;
use crate::particles::lava_particles::lava_particles;
use crate::particles::types::Particle;
use crate::utils::extern_c::get_random;
use crate::{ 
    coins::types::Coin, enemies::{self, climber::climber::Climber, crawler::crawler::Crawler, types::EnemyTrait}, get_map, Player, Tile, Vec2, Vec2usize 
};

pub fn generate_map_collisions(
    origin_x: usize, 
    origin_y: usize, 
    player: &Player,
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
    generate_enemies: bool,
    lava_tiles: &mut Vec<Tile>
) -> (
    HashMap<(usize,usize),Tile>,
    Vec<Tile>
) {
    let mut collisions_map = HashMap::new(); 
    let mut lethal_tiles: Vec<Tile> = vec![];
    let game_map = get_map();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    if generate_enemies {
        *enemies = vec![];
    }
    
    for y in origin_y..origin_y + num_of_tiles {
        if y >= game_map.len() { return (collisions_map,lethal_tiles) }
        for x in origin_x..origin_x + num_of_tiles {
            if x >= game_map[y].len() { return (collisions_map,lethal_tiles) }
            let mut tile = Tile {
                map_origin: player.map_origin.clone(),
                tile_pos: Vec2usize {
                    x: (x % num_of_tiles),
                    y: (y % num_of_tiles)
                },
                position: Vec2 {
                    x: (x % num_of_tiles) as f64 * tile_size,
                    y: (y % num_of_tiles) as f64 * tile_size
                },
                sheet: None,
                touched_by_player: false,
                just_restored: false,
                hanging_tile: false,
                spawning_tile: false,
            };
            if game_map[y][x] == 0 {
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    tile.clone()
                );
            }
            if game_map[y][x] == 20 {
                let mut new_tile = tile.clone();
                new_tile.spawning_tile = true;
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    new_tile
                );
            }
            if game_map[y][x] == 6 {
                tile.sheet = Some(player.sprite_sheets.get("sand").unwrap().clone());
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    tile.clone()
                );
            }
            if game_map[y][x] == 7 {
                let mut new_tile = tile.clone();
                new_tile.hanging_tile = true;
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    new_tile
                );
            }
            if game_map[y][x] == 9 {
                lethal_tiles.push(tile.clone());
                lava_tiles.push(tile.clone());
            }

            if !generate_enemies {
                continue;
            }

            if game_map[y][x] == 2 {
                let random = get_random(0.1, 0.9);
                let sheet_name = "crawler_1_0".to_string();
                let abs_x = (x % num_of_tiles) as f64 * tile_size;
                let abs_y = (y % num_of_tiles) as f64 * tile_size;
                let crawler = Crawler {
                    position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    spritesheet: player.sprite_sheets.get(&sheet_name).unwrap().clone(),
                    sheet_name,
                    initial_position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    limit_position: Vec2 {
                        x: (abs_x - (tile_size * 2.)),
                        y: abs_y
                    },
                    direction: enemies::types::LeftRight::Left,
                    initial_direction: enemies::types::LeftRight::Left,
                    velocity: Vec2 {
                        x: -random,
                        y: 0.,
                    },
                };
                enemies.push(Box::new(crawler));
            }
            if game_map[y][x] == 3 {
                let random = get_random(0.1, 0.5);
                let sheet_name = "climber_R".to_string();
                let abs_x = (x % num_of_tiles) as f64 * tile_size;
                let abs_y = (y % num_of_tiles) as f64 * tile_size;
                let climber = Climber {
                    facing_right: true,
                    spritesheet: player.sprite_sheets.get(&sheet_name).unwrap().clone(),
                    sheet_name,
                    position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    initial_position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    limit_position: Vec2 {
                        x: abs_x,
                        y: (abs_y + (tile_size * 2.)),
                    },
                    direction: enemies::types::UpDown::Down,
                    initial_direction: enemies::types::UpDown::Down,
                    velocity: Vec2 {
                        x: 0.,
                        y: random,
                    },
                };
                enemies.push(Box::new(climber));
            }
            if game_map[y][x] == 4 {
                let random = get_random(0.1, 0.5);
                let sheet_name = "climber_L".to_string();
                let abs_x = (x % num_of_tiles) as f64 * tile_size;
                let abs_y = (y % num_of_tiles) as f64 * tile_size;
                let climber = Climber {
                    facing_right: false,
                    spritesheet: player.sprite_sheets.get(&sheet_name).unwrap().clone(),
                    sheet_name,
                    position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    initial_position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
                    limit_position: Vec2 {
                        x: abs_x,
                        y: (abs_y + (tile_size * 2.)),
                    },
                    direction: enemies::types::UpDown::Down,
                    initial_direction: enemies::types::UpDown::Down,
                    velocity: Vec2 {
                        x: 0.,
                        y: random,
                    },
                };
                enemies.push(Box::new(climber));
            }
        }
    }
    (collisions_map,lethal_tiles)
}
