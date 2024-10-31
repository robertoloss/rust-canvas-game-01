use crate::PLAYER;
use crate::HtmlImageElement;
use crate::ThreadSafeImage;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_tile_image(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.tile_image = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_lava_sheet(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.lava_sheet.sheet = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_left = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_cling(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_cling = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_image_cling_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.player_image_cling_left = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_sheet_run_right(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.run_right.sheet = ThreadSafeImage(img.map(|i| i.into()));
}
#[wasm_bindgen]
pub fn set_player_sheet_run_left(img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    player.run_left.sheet = ThreadSafeImage(img.map(|i| i.into()));
}
