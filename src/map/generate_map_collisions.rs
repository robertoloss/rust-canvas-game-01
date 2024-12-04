use wasm_bindgen::UnwrapThrowExt;
use web_sys::console;

use crate::{ get_map, HashMap, Player, SpriteSheet, Tile, Vec2, Vec2usize };

pub fn generate_map_collisions(
    origin_x: usize, 
    origin_y: usize, 
    player: &Player
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
        }
    }
    (collisions_map,lethal_tiles)
}
