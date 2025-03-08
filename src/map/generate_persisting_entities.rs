use crate::{coins::types::Coin, get_map, log_out_f, Player, Vec2, Vec2usize};


pub fn generate_persisting_entities(
    coins: &mut Vec<Coin>,
    player: &Player
) {
    let game_map = get_map();
    if game_map.len() == 0 {
        return;
    }
    let num_of_tiles = player.screen_tiles;
    let tile_size = player.tile_size;
    log_out_f(18 - (18 % num_of_tiles));

    for y in 0..game_map.len() {
        for x in 0..game_map[0].len() {
            match game_map[y][x] {
                12 => {
                    coins.push(Coin {
                        map_origin: Vec2usize { 
                            x: x - (x % num_of_tiles),
                            y: y - (y % num_of_tiles),
                        },
                        position: Vec2 {
                            x: (x % num_of_tiles) as f64 * tile_size,
                            y: (y % num_of_tiles) as f64 * tile_size
                        },
                        active: true,
                        show_plus_one: false,
                        counter: 0,
                    });
                }
                _ => {}
            }
        }
    }
}
