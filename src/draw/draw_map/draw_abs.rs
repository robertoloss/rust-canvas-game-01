use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::{Player, SpriteSheet};

pub fn draw_abs(
    sheet: &HtmlImageElement,
    sprite_sheet: &SpriteSheet,
    ctx: &CanvasRenderingContext2d,
    player: &Player,
    x: f64,
    y: f64
) 
    -> Result<(), JsValue> 
{
    let tile_size = player.tile_size;

    ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &sheet,
        0., 
        sprite_sheet.tile_position_pointer_y * tile_size,
        tile_size, 
        tile_size,
        x,
        y,
        tile_size, 
        tile_size,
    )
}
