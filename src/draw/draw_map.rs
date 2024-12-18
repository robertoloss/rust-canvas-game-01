mod draw_this;
mod draw_this_sw_sh;
mod draw_sand;
use std::collections::HashMap;
use crate::JsValue;
use draw_sand::draw_sand;
use draw_this::draw_this;
use draw_this_sw_sh::draw_this_sw_sh;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::{get_map, Player, Tile};

pub fn draw_map(
    player: &mut Player,
    ctx: &CanvasRenderingContext2d,
    collision_map: &mut HashMap<(usize, usize), Tile>,
) 
    -> Result<(), JsValue> 
{
    let tile: HtmlImageElement = player.images.get("tile")
        .unwrap().0.clone().unwrap().into();
    let hang: HtmlImageElement = player.images.get("hang")
        .unwrap().0.clone().unwrap().into();
    let lava_sheet: HtmlImageElement = player.sprite_sheets.get("lava")
        .unwrap().sheet.0.clone().unwrap().into();
    let game_map = get_map();
    let num_of_tiles = player.screen_tiles;

    for y in player.map_origin.y..player.map_origin.y + num_of_tiles {
        for x in player.map_origin.x..player.map_origin.x + num_of_tiles {
            let lava_sprite_sheet = player.sprite_sheets.get("lava").unwrap();
            match game_map[y][x] {
                0 => draw_this(&tile, ctx, player, x, y)?,
                7 => draw_this(&hang, ctx, player, x, y)?,
                9 => draw_this_sw_sh(&lava_sheet, &lava_sprite_sheet, ctx, player, x, y)?,
                6 => draw_sand(player, ctx, collision_map, x, y)?,
                _ => {}
             } 
        }
    }
    Ok(())
}

