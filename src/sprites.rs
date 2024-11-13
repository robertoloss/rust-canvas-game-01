use crate::PLAYER;
use crate::HtmlImageElement;
use crate::ThreadSafeImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_image(name: String, sheet: bool, img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    let image = ThreadSafeImage(img.map(|i| i.into()));
    if !sheet {
        player.images.insert(name, image);
    } else {
        if let Some(sheet) = player.sprite_sheets.get_mut(&name) {
            sheet.sheet = image;
        }
    }
}
