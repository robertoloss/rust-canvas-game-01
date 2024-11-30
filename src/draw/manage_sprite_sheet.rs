use crate::SpriteSheet;

pub fn manage_sprite_sheet <F>(
    sheet: &mut SpriteSheet,
    step: f64,
    limit: f64,
    mut action: F
)
    where F: FnMut()
{
    sheet.counter +=1;
    if sheet.counter > sheet.counter_limit {
        sheet.counter = 0;
        sheet.tile_position_pointer_y += step;
        if sheet.tile_position_pointer_y == limit {
            action()
        }
    }
}
