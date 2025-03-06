use web_sys::CanvasRenderingContext2d;
use crate::{screen_size, Player};

pub fn debug(ctx: &CanvasRenderingContext2d, player: &Player) {
    ctx.set_font("20px Arial, sans-serif");
    ctx.set_fill_style_str("yellow");
    let _ = ctx.fill_text(
        &("is_clinging = ".to_owned() + &player.is_clinging.to_string()).to_string(), 
        32., 24.
    );
    let _ = ctx.fill_text(
        &("can_cling = ".to_owned() + &player.can_cling.to_string()).to_string(), 
        32., 48.
    );
    let _ = ctx.fill_text(
        &("wants_to_cling = ".to_owned() + &player.wants_to_cling.to_string()).to_string(), 
        32., 72.
    );
    let _ = ctx.fill_text(
        &("clinging_tile_coord = ".to_owned() 
            + &player.clinging_tile_coord.unwrap().0.to_string()
            + " "
            + &player.clinging_tile_coord.unwrap().1.to_string()
        ).to_string(), 
        32., 96.
    );
    let _ = ctx.fill_text(&format!("Screen size: {}", screen_size()), 32., 120.);
}
