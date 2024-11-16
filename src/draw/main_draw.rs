use crate::get_map;
use crate::HtmlImageElement;
use crate::get_context;
use wasm_bindgen::JsValue;
use crate::ThreadSafeImage;
use crate::Player;

pub fn main_draw(
    player: &mut Player,
    tile_size: f64,
    num_of_tiles: usize,
) -> Result<(), JsValue> 
{
    let image: HtmlImageElement = player.images.get("tile").unwrap().0.clone().unwrap().into();
    let lava_sheet: HtmlImageElement = player.sprite_sheets.get("lava").unwrap().sheet.0.clone().unwrap().into();
    let game_map = get_map();
    match get_context(&(*player)) {
        Ok((context, canvas)) => {
            let ctx = &context;
            ctx.set_image_smoothing_enabled(false);
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

            for y in player.map_origin.y..player.map_origin.y + num_of_tiles {
                for x in player.map_origin.x..player.map_origin.x + num_of_tiles {
                    let lava_sprite_sheet = player.sprite_sheets.get_mut("lava").unwrap();
                     match game_map[y][x] {
                         0 => ctx.draw_image_with_html_image_element_and_dw_and_dh(
                            &image,
                            (x % num_of_tiles) as f64 * tile_size, 
                            (y % num_of_tiles) as f64 * tile_size, 
                            tile_size,
                            tile_size,
                        )?,
                        9 => ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                            &lava_sheet,
                            0., 
                            lava_sprite_sheet.tile_position_pointer_y * tile_size,
                            tile_size, 
                            tile_size,
                            (x % num_of_tiles) as f64 * tile_size, 
                            (y % num_of_tiles) as f64 * tile_size, 
                            tile_size, 
                            tile_size,
            )?,
                        _ => {}
                     } 
                }
            }
            let lava_sprite_sheet = player.sprite_sheets.get_mut("lava").unwrap();
            lava_sprite_sheet.counter += 1;
            if lava_sprite_sheet.counter > lava_sprite_sheet.counter_limit {
                lava_sprite_sheet.counter = 0;
                lava_sprite_sheet.tile_position_pointer_y += 1.;
                if lava_sprite_sheet.tile_position_pointer_y * tile_size >= lava_sprite_sheet.pointer_y_limit {
                    lava_sprite_sheet.tile_position_pointer_y = 0.
                }
            }

            if player.is_dead {
                let death_sheet: HtmlImageElement = player.sprite_sheets
                    .get("death")
                    .unwrap()
                    .sheet
                    .0
                    .clone()
                    .unwrap()
                    .into();

                ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &death_sheet,
                    0., player.sprite_sheets.get("death").unwrap().tile_position_pointer_y * tile_size, 
                    tile_size, 
                    tile_size,
                    player.position.x, 
                    player.position.y, 
                    tile_size, 
                    tile_size,
                )?;
                player.sprite_sheets.get_mut("death").unwrap().counter += 1;
                if player.sprite_sheets.get_mut("death").unwrap().counter > player.sprite_sheets.get("death").unwrap().counter_limit {
                    player.sprite_sheets.get_mut("death").unwrap().counter = 0;
                    player.sprite_sheets.get_mut("death").unwrap().tile_position_pointer_y += 1.;
                    if player.sprite_sheets.get_mut("death").unwrap().tile_position_pointer_y * player.tile_size >= player.sprite_sheets.get("death").unwrap().pointer_y_limit {
                        //player.death_sheet.tile_position_pointer_y = 0.;
                        player.position = player.position_spawn.clone();
                        player.is_dead = false;
                        player.sprite_sheets.get_mut("death").unwrap().tile_position_pointer_y = 0.;
                        return Ok(())
                    }
                }


                return  Ok(())
            }
            
            //ctx.set_font("14px Arial, sans-serif");
            //ctx.set_fill_style(&JsValue::from_str("yellow"));
            //let _ = ctx.fill_text(&player.delta.to_string(), 30., 15.);
            //let _ = ctx.fill_text(&delta.to_string(), 30., 30.);
            if player.is_clinging || (player.velocity.x == 0. || player.velocity.y != 0.) {
                player.sprite_counter = 0;
            }

            let mut _image: &ThreadSafeImage = &ThreadSafeImage(None); 
            player.sprite_counter += 1;

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
            //console::log_1(&JsValue::from_str(&format!("{}", player.sprite_counter)));

            let player_sprite = _image.0.clone().unwrap().into();
            let is_run_right_sheet = std::ptr::eq(_image, 
                   &player.sprite_sheets.get("player_run_right").unwrap().sheet
);
            let is_run_left_sheet = std::ptr::eq(_image, 
                   &player.sprite_sheets.get("player_run_left").unwrap().sheet
);

            if is_run_right_sheet {
                if player.sprite_counter >= player.sprite_sheets.get("player_run_right").unwrap().counter_limit {
                    player.sprite_counter = 0;
                    player.sprite_sheets.get_mut("player_run_right").unwrap().pointer_y += tile_size;
                    if player.sprite_sheets.get("player_run_right").unwrap().pointer_y >= player.sprite_sheets.get("player_run_right").unwrap().pointer_y_limit {
                        player.sprite_sheets.get_mut("player_run_right").unwrap().pointer_y = 0.
                    }
                }
                pointer_y = player.sprite_sheets.get("player_run_right").unwrap().pointer_y;
            } else if is_run_left_sheet {
                if player.sprite_counter >= player.sprite_sheets.get("player_run_left").unwrap().counter_limit {
                    player.sprite_counter = 0;
                    player.sprite_sheets.get_mut("player_run_left").unwrap().pointer_y += tile_size;
                    if player.sprite_sheets.get("player_run_left").unwrap().pointer_y >= player.sprite_sheets.get("player_run_right").unwrap().pointer_y_limit {
                        player.sprite_sheets.get_mut("player_run_left").unwrap().pointer_y = 0.
                    }
                }
                pointer_y = player.sprite_sheets.get("player_run_left").unwrap().pointer_y;
            }
            ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &player_sprite,
                0., pointer_y, tile_size, tile_size,
                player.position.x, player.position.y, tile_size, tile_size,
            )?;
        },
        Err(e) => eprintln!("Error getting context: {:?}", e)
    }
    Ok(())
}
