use crate::SpriteSheet;

pub fn manage_sprite_sheet <F>(
    sheet: &mut SpriteSheet,
    step: f64,
    limit: f64,
    mut action: F,
    limit_eq: bool,
    tile_size: f64,
)
    where F: FnMut()
{
    sheet.counter +=1;
    if sheet.counter > sheet.counter_limit {
        sheet.counter = 0;
        sheet.tile_position_pointer_y += step;
        if limit_eq {
            if sheet.tile_position_pointer_y * tile_size == limit {
                action();
            }
        } else {
            if sheet.tile_position_pointer_y * tile_size >= limit {
                action()
            }
        }
    }
}
