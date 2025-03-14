use std::collections::HashMap;
use crate::utils::extern_c::play;
use crate::Vec2usize;
use crate::TileToRestore;
use crate::draw::manage_sprite_sheet::manage_sprite_sheet;
use crate::JsValue;
use crate::Tile;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlImageElement;
use crate::Player;

pub fn draw_sand(
    player: &mut Player,
    ctx: &CanvasRenderingContext2d,
    collision_map: &mut HashMap<(usize, usize), Tile>,
    x: usize,
    y: usize
)
    -> Result<(), JsValue>
{
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;
    let sand_sheet: HtmlImageElement = player.sprite_sheets.get("sand")
        .unwrap().sheet.0.clone().unwrap().into();
    let sand_tile_option = collision_map.get_mut(&(
            x - player.map_origin.x, 
            y - player.map_origin.y
        ));
    if sand_tile_option.is_some() {
        let sand_tile = sand_tile_option.unwrap();
        //console::log_1(&format!("SAND {:?}", sand_tile).into());
        if let Some(sand_sprite_sheet) = &mut sand_tile.sheet {
            ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &sand_sheet,
                0., 
                sand_sprite_sheet.tile_position_pointer_y * tile_size,
                tile_size, 
                tile_size,
                (x % num_of_tiles) as f64 * tile_size, 
                (y % num_of_tiles) as f64 * tile_size, 
                tile_size, 
                tile_size,
            )?;
            if sand_tile.just_restored {
                manage_sprite_sheet(
                    sand_sprite_sheet, 
                    -1., 
                    0., 
                    Some(|| { sand_tile.just_restored = false; }),
                    tile_size
                );
            }
            if sand_tile.touched_by_player && !sand_tile.just_restored {
                if !player.sound_playing.get("sand").unwrap() {
                    play("sand");
                    player.sound_playing.insert("sand".into(), true);
                };
                sand_sprite_sheet.counter += 1;
                if sand_sprite_sheet.counter > sand_sprite_sheet.counter_limit {
                    sand_sprite_sheet.counter = 0;
                    sand_sprite_sheet.tile_position_pointer_y += 1.;
                    if sand_sprite_sheet.tile_position_pointer_y * tile_size >= sand_sprite_sheet.pointer_y_limit {
                        collision_map.remove(&(
                            x - player.map_origin.x, 
                            y - player.map_origin.y
                        ));
                        player.sound_playing.insert("sand".into(), false);
                        let tile_to_restore = TileToRestore {
                            tile_coordinates: Vec2usize { x, y },
                            counter: 0,
                            counter_limit: player.time_to_restore,
                            remove_tile: false,
                        };
                        player.tiles_to_restore.push(tile_to_restore)
                    }
                }
            }
        }
    }
    Ok(())
}
