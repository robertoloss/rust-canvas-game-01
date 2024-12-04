use std::collections::HashMap;
use web_sys::console;

use crate::{collisions::normal_tile_collision::normal_tile_collision, Player, Tile, Vec2, Vec2usize};

pub fn restore_sand_tiles(
    player: &mut Player,
    collision_map: &mut HashMap<(usize, usize), Tile>
) {
    let tile_size = player.tile_size;
    let origin_x = player.map_origin.x.clone();
    let origin_y = player.map_origin.y.clone();

    let sheet = Some(player.sprite_sheets.get("sand").unwrap().clone()); 
    let tiles_to_restore = &mut player.tiles_to_restore;
    for i in (0..tiles_to_restore.len()).rev() {
        let tile = &player.tiles_to_restore[i];
        if tile.counter >= tile.counter_limit {
            let mut cloned_sheet = sheet.clone().unwrap();
            cloned_sheet.tile_position_pointer_y = 6.;
            let new_tile = Tile {
                tile_pos: Vec2usize {
                    x: tile.tile_coordinates.x - origin_x,
                    y: tile.tile_coordinates.y - origin_y,
                },
                position: Vec2 {
                    x: (tile.tile_coordinates.x - origin_x) as f64 * tile_size,
                    y: (tile.tile_coordinates.y - origin_y) as f64 * tile_size
                },
                sheet: Some(cloned_sheet),
                touched_by_player: false,
                just_restored: true,
                hanging_tile: false,
            };
            let overlap = normal_tile_collision(&new_tile, player);
            if !overlap {
                collision_map.insert(
                    ( 
                        tile.tile_coordinates.x - origin_x, 
                        tile.tile_coordinates.y - origin_y
                    ), 
                    new_tile.clone()
                );
                let tile = &mut player.tiles_to_restore[i];
                tile.remove_tile = true;
            }
        } else {
            let tile = &mut player.tiles_to_restore[i];
            tile.counter += 1;
        }
    }
    let tiles_to_restore2 = &mut player.tiles_to_restore;
    tiles_to_restore2.retain(|tile| {
        if tile.remove_tile {
            false
        } else {
            true
        }
    });

}
