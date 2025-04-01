use crate::player::types::Player;


pub fn get_tiles_around_player(
    player: &mut Player
) 
    -> [(usize,usize); 4] 
{
    let tile_size = player.tile_size;
    let cling_offset = -0.1;
    return [
        (
            ((player.position.x + player.velocity.x + tile_size - player.hitbox.right + cling_offset) / 
                tile_size).floor() as usize,
            ((player.position.y + player.velocity.y) / tile_size).floor() as usize
        ),
        (
            ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
            ((player.position.y + player.velocity.y) / tile_size).floor() as usize
        ),
        (
            ((player.position.x + player.velocity.x + tile_size - player.hitbox.right + cling_offset) /
                tile_size).floor() as usize,
            ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
        ),
        (
            ((player.position.x + player.velocity.x + player.hitbox.left) / tile_size).floor() as usize,
            ((player.position.y + player.velocity.y + tile_size) / tile_size).floor() as usize
        ),
    ]

}
