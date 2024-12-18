use std::usize;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::{Player, SpriteSheet};

pub fn draw_this_sw_sh(
    sheet: &HtmlImageElement,
    sprite_sheet: &SpriteSheet,
    ctx: &CanvasRenderingContext2d,
    player: &Player,
    x: usize,
    y: usize
) 
    -> Result<(), JsValue> 
{
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;

    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &sheet,
        0., 
        sprite_sheet.tile_position_pointer_y * tile_size,
        tile_size, 
        tile_size,
        (x % num_of_tiles) as f64 * tile_size, 
        (y % num_of_tiles) as f64 * tile_size, 
        tile_size, 
        tile_size,
    )
}
