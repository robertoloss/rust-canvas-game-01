use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::coins::types::Coin;
use crate::enemies;
use crate::enemies::types::EnemyTrait;
use crate::generate_map_collisions;
use crate::log_out_f;
use crate::HtmlImageElement;
use crate::Player;
use crate::Tile;
use crate::Vec2;

pub fn manage_death(
    player: &mut Player,
    ctx: &CanvasRenderingContext2d,
    collision_map: &mut HashMap<(usize, usize), Tile>,
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
    coins: &mut Vec<Coin>
) 
    -> Result<(), JsValue>
{
    let tile_size = player.tile_size;
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
            player.velocity = Vec2 { x: 0., y: 0. }; 
            player.moves.jump = false;
            player.tiles_to_restore = vec![];

            player.map_origin = player.map_origin_spawn.clone();

            *collision_map = generate_map_collisions(
                player.map_origin.x, 
                player.map_origin.y, 
                player,
                enemies,
                true,
            ).0;
            player.map_origin = player.map_origin_spawn.clone();
            player.is_dead = false;
            player.sprite_sheets.get_mut("death").unwrap().tile_position_pointer_y = 0.;
            return Ok(())
        }
    }
    return  Ok(())
}
