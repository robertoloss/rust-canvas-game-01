use std::collections::HashMap;
use crate::enemies::types::EnemyTrait;
use crate::enemies::types::LeftRight;
use crate::get_context;
use crate::log_out_f;
use crate::Tile;
use crate::ENEMIES;
use draw_abs::draw_abs;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;
use crate::ThreadSafeImage;
use crate::Player;
use super::debug::debug;
use super::manage_death::manage_death;
use super::manage_sprite_sheet::manage_sprite_sheet;
use super::do_draw_map::do_draw_map;
use super::draw_map::*;

pub fn main_draw(
    collision_map: &mut HashMap<(usize, usize), Tile>,
    player: &mut Player,
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
) 
    -> Result<(), JsValue> 
{
    let tile_size = player.tile_size;
    match get_context(&(*player)) {
        Ok((context, canvas)) => {
            let ctx = &context;
            ctx.set_image_smoothing_enabled(false);
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            ctx.set_fill_style_str(&"black");
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            do_draw_map(
                player,
                ctx,
                collision_map
            )?;

            let mut lava_sprite_sheet = player.sprite_sheets.get_mut("lava").unwrap();
            let lava_pointer_y_limit = lava_sprite_sheet.pointer_y_limit;
            manage_sprite_sheet::<fn()>(
                &mut lava_sprite_sheet,
                1.0,
                lava_pointer_y_limit,
                None,
                tile_size
            );

            for enemy in enemies.iter_mut() {
                let position = enemy.position();
                let sheet_name = enemy.get_sheetname();
                let sheet = enemy.get_spritesheet();
                let limit = sheet.pointer_y_limit.clone();

                manage_sprite_sheet::<fn()>(
                    sheet,
                    1.0,
                    limit,
                    None,
                    tile_size
                );

                if let Some(img_sheet) = player.sprite_sheets.get_mut(&sheet_name).unwrap().sheet.0.clone() {
                    draw_abs(
                        &img_sheet, 
                        sheet, 
                        ctx, 
                        player, 
                        position.x,
                        position.y
                    )?
                }
            }

            if player.is_dead {
                return manage_death(player, &ctx, collision_map, enemies)
            }
            
            if player.show_debug { debug(ctx, player) };

            if  player.is_clinging || (player.velocity.x == 0. || player.velocity.y != 0.) {
                player.sprite_counter = 0;
            }

            let mut _image: &ThreadSafeImage = &ThreadSafeImage(None); 
            player.sprite_counter += 1;

            if !player.is_hanging {
                match player.facing_right {
                    true => _image =  if player.is_clinging {
                        &player.images.get("player_cling").unwrap()
                    } else if player.velocity.x == 0. || player.velocity.y != 0. { 
                        &player.images.get("player").unwrap()
                    } else {
                       &player.sprite_sheets.get("player_run_right").unwrap().sheet
                    },
                    false => _image = if player.is_clinging {
                        &player.images.get("player_cling_left").unwrap()
                    } else if player.velocity.x == 0. || player.velocity.y != 0. {
                        &player.images.get("player_left").unwrap()
                    } else {
                       &player.sprite_sheets.get("player_run_left").unwrap().sheet
                    }
                }
                let mut pointer_y = 0.;

                let player_sprite = _image.0.clone().unwrap().into();
                let is_run_right_sheet = std::ptr::eq(_image, 
                       &player.sprite_sheets.get("player_run_right").unwrap().sheet
                );
                let is_run_left_sheet = std::ptr::eq(_image, 
                       &player.sprite_sheets.get("player_run_left").unwrap().sheet
                );

                if is_run_right_sheet {
                    let sheet = player.sprite_sheets.get_mut("player_run_right").unwrap();
                    let limit = sheet.pointer_y_limit;
                    manage_sprite_sheet::<fn()>(
                        sheet,
                        1.0,
                        limit,
                        None,
                        tile_size
                    );
                    pointer_y = sheet.tile_position_pointer_y;
                } else if is_run_left_sheet {
                    let sheet = player.sprite_sheets.get_mut("player_run_left").unwrap();
                    let limit = sheet.pointer_y_limit;
                    manage_sprite_sheet::<fn()>(
                        sheet,
                        1.0,
                        limit,
                        None,
                        tile_size
                    );
                    pointer_y = sheet.tile_position_pointer_y;
                }
                ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &player_sprite,
                    0., pointer_y * tile_size, tile_size, tile_size,
                    player.position.x, player.position.y, tile_size, tile_size,
                )?;
            } 
            else {
                match player.facing_right {
                    true => _image = if player.velocity.x == 0. { 
                            &player.images.get("cling_still_R").unwrap()
                        } else {
                           &player.sprite_sheets.get("cling_move_R").unwrap().sheet
                        },
                    false => _image = if player.velocity.x == 0. {
                        &player.images.get("cling_still_L").unwrap()
                    } else {
                       &player.sprite_sheets.get("cling_move_L").unwrap().sheet
                    }
                }

                let mut pointer_y = 0.;

                let player_sprite = _image.0.clone().unwrap().into();
                let is_run_right_sheet = std::ptr::eq(_image, 
                       &player.sprite_sheets.get("cling_move_R").unwrap().sheet
    );
                let is_run_left_sheet = std::ptr::eq(_image, 
                       &player.sprite_sheets.get("cling_move_L").unwrap().sheet
    );

                if is_run_right_sheet {
                    if player.sprite_counter >= player.sprite_sheets.get("cling_move_R").unwrap().counter_limit {
                        player.sprite_counter = 0;
                        player.sprite_sheets.get_mut("cling_move_R").unwrap().pointer_y += tile_size;
                        if player.sprite_sheets.get("cling_move_R").unwrap().pointer_y >= player.sprite_sheets.get("cling_move_R").unwrap().pointer_y_limit {
                            player.sprite_sheets.get_mut("cling_move_R").unwrap().pointer_y = 0.
                        }
                    }
                    pointer_y = player.sprite_sheets.get("cling_move_R").unwrap().pointer_y;
                } else if is_run_left_sheet {
                    if player.sprite_counter >= player.sprite_sheets.get("cling_move_L").unwrap().counter_limit {
                        player.sprite_counter = 0;
                        player.sprite_sheets.get_mut("cling_move_L").unwrap().pointer_y += tile_size;
                        if player.sprite_sheets.get("cling_move_L").unwrap().pointer_y >= player.sprite_sheets.get("cling_move_L").unwrap().pointer_y_limit {
                            player.sprite_sheets.get_mut("cling_move_L").unwrap().pointer_y = 0.
                        }
                    }
                    pointer_y = player.sprite_sheets.get("cling_move_L").unwrap().pointer_y;
                }
                ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &player_sprite,
                    0., 
                    pointer_y, 
                    tile_size, 
                    tile_size,
                    player.position.x, 
                    player.position.y, 
                    tile_size, 
                    tile_size,
                )?;
            }

        },
        Err(e) => eprintln!("Error getting context: {:?}", e)
    }
    Ok(())
}
