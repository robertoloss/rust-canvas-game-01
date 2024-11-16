use crate::{ Player, HashMap, Tile, Vec2, get_map, Vec2usize };

pub fn generate_map_collisions(origin_x: usize, origin_y: usize, player: &Player) -> (HashMap<(usize,usize), Tile>,Vec<Tile>) {
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
            let tile = Tile {
                tile_pos: Vec2usize {
                    x: (x % num_of_tiles),
                    y: (y % num_of_tiles)
                },
                position: Vec2 {
                    x: (x % num_of_tiles) as f64 * tile_size,
                    y: (y % num_of_tiles) as f64 * tile_size
                }
            };
            if game_map[y][x] == 0 {
                collisions_map.insert(
                    ( (x % num_of_tiles) , (y % num_of_tiles) ), 
                    tile.clone()
                );
            }
            if game_map[y][x] == 9 {
                lethal_tiles.push(tile)
            }
        }
    }
    (collisions_map,lethal_tiles)
}
