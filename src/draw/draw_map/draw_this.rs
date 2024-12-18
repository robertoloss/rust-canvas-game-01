use std::usize;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::Player;

pub fn draw_this(
    tile: &HtmlImageElement,
    ctx: &CanvasRenderingContext2d,
    player: &mut Player,
    x: usize,
    y: usize
) 
    -> Result<(), JsValue> 
{
    let tile_size = player.tile_size;
    let num_of_tiles = player.screen_tiles;

    ctx.draw_image_with_html_image_element_and_dw_and_dh(
        tile,
        (x % num_of_tiles) as f64 * tile_size, 
        (y % num_of_tiles) as f64 * tile_size, 
        tile_size,
        tile_size,
    )
}
