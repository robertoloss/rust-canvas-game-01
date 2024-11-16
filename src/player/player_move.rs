use crate::Player;
use crate::collisions;

pub fn player_move(player: &mut Player, delta: f64) {
    player.velocity.x = if player.moves.right { 
    player.horizontal_velocity 
    } else if player.moves.left { 
        -player.horizontal_velocity
    } else { 0. };
    //player.velocity.y = if player.moves.down { 4.0 } else if player.moves.up { -4.0 } else { 0. };
    if !player.is_clinging {
        if player.velocity.x > 0. {
            player.facing_right = true
        }
        if player.velocity.x < 0. {
            player.facing_right = false
        }
    }
    if player.moves.jump {
        player.moves.jump = false;
        player.velocity.y = player.jump_velocity; //-10.1
    }
    if player.moves.stop_jump {
        player.moves.stop_jump = false;
        if player.velocity.y < -3. {
            player.velocity.y += 3.//3.
        }
    }
    if player.velocity.y < player.max_fall_velocity {
        player.velocity.y += player.gravity / delta
    }
    if player.wants_to_cling && player.can_cling != collisions::LeftRight::None {
        player.is_clinging = true
    }
    if player.is_clinging {
        player.velocity.y = 0.;
        player.velocity.x = 0.;
    }
    player.position.x += player.velocity.x / delta;
    player.position.y += player.velocity.y / delta;
}
