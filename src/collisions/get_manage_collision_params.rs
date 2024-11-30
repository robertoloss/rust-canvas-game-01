use crate::collisions::types::*;
use crate::Player;

pub fn get_manage_collision_parameters(
    up_down: UpDown, 
    left_right: LeftRight, 
    player: &Player
) -> (f64, f64, f64, f64, f64, f64) 
{
    let tile_size = player.tile_size;

    let off_tile_x = if let LeftRight::Left = left_right { 
        tile_size - player.hitbox.left
    } else { 
        -tile_size + player.hitbox.right
    };

    let off_tile_y = if let UpDown::Up = up_down {
        tile_size
    } else {
        -tile_size
    };

    let off_tile_x_intersection = if let LeftRight::Left = left_right {
        tile_size
    } else {
        0.
    };

    let off_tile_y_intersection = if let UpDown::Up = up_down {
        tile_size
    } else {
        0.
    };

    let off_player_x = if let LeftRight::Left = left_right { 
        player.hitbox.left
    } else { 
        tile_size - player.hitbox.right
    };

    let off_player_y = if let UpDown::Up = up_down {
        0.
    } else {
        tile_size
    };

    (
        off_tile_x,
        off_tile_y,
        off_tile_x_intersection,
        off_tile_y_intersection,
        off_player_x,
        off_player_y
    )
}
