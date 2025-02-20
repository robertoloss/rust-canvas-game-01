use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::console;

use crate::{ enemies::{self, types::{Crawler, EnemyTrait}}, get_map, log_out_f, HashMap, Player, SpriteSheet, Tile, Vec2, Vec2usize, ENEMIES };

pub fn generate_map_collisions(
    origin_x: usize, 
    origin_y: usize, 
    player: &Player,
    enemies: &mut Vec<Box<dyn EnemyTrait>>
) -> (
    HashMap<(usize,usize), 
    Tile>,
    Vec<Tile>
) {
    let mut collisions_map = HashMap::new(); 
    let mut lethal_tiles: Vec<Tile> = vec![];
    let game_map = get_map();
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    *enemies = vec![];
    
    if game_map.len() == 0 {
        return (collisions_map,lethal_tiles)
    }
    for y in origin_y..origin_y + num_of_tiles {
        for x in origin_x..origin_x + num_of_tiles {
            let mut tile = Tile {
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
            };
            if game_map[y][x] == 0 {
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    tile.clone()
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
                lethal_tiles.push(tile.clone())
            }
            if game_map[y][x] == 2 {
                let abs_x = (x % num_of_tiles) as f64 * tile_size;
                let abs_y = (y % num_of_tiles) as f64 * tile_size;
                let crawler = Crawler {
                    position: Vec2 {
                        x: abs_x,
                        y: abs_y
                    },
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
                        x: -0.5,
                        y: 0.,
                    },
                    spritesheet: player.sprite_sheets.get("crawler_1_0").unwrap().clone(),
                };
                enemies.push(Box::new(crawler));
            }
        }
    }
    (collisions_map,lethal_tiles)
}
