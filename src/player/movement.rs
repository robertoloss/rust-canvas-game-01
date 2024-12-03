use crate::PLAYER;
use crate::wasm_bindgen;

#[wasm_bindgen]
pub fn movement(key_code: i32) {
    let mut player = PLAYER.lock().unwrap();
    match key_code {
        0 => {
            player.moves.left = true;
            player.moves.right = false;
            if !player.is_clinging {
                player.facing_right = false;
            }
        },
        1 => {
            player.moves.right = true;
            player.moves.left = false;
            if !player.is_clinging {
                player.facing_right = true;
            }
        },
        2 => {
            player.moves.jump = true;
            player.moves.stop_jump = false;
            player.moves.airborne = true;
            player.wants_to_cling = false;
            if player.is_clinging {
                player.is_clinging = false;
                if player.moves.right || player.moves.left {
                    player.facing_right = if player.moves.right {
                        true
                    } else {
                        false
                    }
                }
            }
        },
        3 => {
            player.wants_to_cling = true;
        }
        4 => {
            player.show_debug = !player.show_debug;
        }
        _ => {}
    }
}
