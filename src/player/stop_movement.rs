use crate::PLAYER;
use crate::wasm_bindgen;

#[wasm_bindgen]
pub fn stop_movement(key_code: i32) {
    //console::log_1(&JsValue::from_str(&format!("key_code: {}", key_code)));
    let mut player = PLAYER.lock().unwrap();
    match key_code {
        0 => {
            player.moves.left = false;
        },
        1 => {
            player.moves.right = false;
        },
        2 => {
            player.moves.stop_jump = true;
        },
        3 => {
            player.wants_to_cling = false;
            player.is_clinging = false;
        }
        _ => {}
    }
}
