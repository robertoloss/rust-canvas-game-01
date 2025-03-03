use std::fmt::Debug;

use crate::{ enemies, wasm_bindgen, CanvasRenderingContext2d, HashMap, HtmlCanvasElement, JsValue, Player, SpriteSheet, ThreadSafeImage, ENEMIES, PLAYER };
use wasm_bindgen::JsCast;
use web_sys::console;
use crate::HtmlImageElement;

#[wasm_bindgen]
pub fn set_image(name: String, sheet: bool, img: Option<HtmlImageElement>) {
    let mut player = PLAYER.lock().unwrap();
    let image = ThreadSafeImage(img.map(|i| i.into()));
    if !sheet {
        player.images.insert(name, image);
    } else {
        if let Some(sheet) = player.sprite_sheets.get_mut(&name) {
            sheet.sheet = image;
        } else {
            console::log_1(&JsValue::from_str(&format!("There was a problem while loading an image")))
        }
    }
}

#[wasm_bindgen]
pub fn get_and_give_f64(num: Option<f64>) {
    //console::log_1(&JsValue::from_str(&format!("{}", num)));
    let mut player = PLAYER.lock().unwrap();
    match num {
        Some(n) => player.delta = n,
        None => player.delta = 60.
    }
}

pub fn all_images_present(images: &HashMap<String, ThreadSafeImage>) -> bool {
    images.values().all(|image| image.0.is_some())
}
pub fn all_sprite_sheets_present(sheets: &HashMap<String, SpriteSheet>) -> bool {
    sheets.values().all(|sheet| sheet.sheet.0.is_some())
}

pub fn get_context(player: &Player) -> Result<(CanvasRenderingContext2d,HtmlCanvasElement), JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("should have a document on window")?;
    let canvas = document.get_element_by_id("gameCanvas").ok_or("can't find gameCanvas")?;
    
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width((player.screen_tiles * (player.tile_size as usize)) as u32);
    canvas.set_height((player.screen_tiles * (player.tile_size as usize)) as u32);
    let context = canvas.get_context("2d")?.ok_or("couldn't get 2D context")?;
    let context: CanvasRenderingContext2d = context.dyn_into::<CanvasRenderingContext2d>()?;

    Ok((context,canvas))
}


pub fn log_out_s(s: &str) {
    console::log_1(&JsValue::from_str(s))
}
pub fn log_out_f<T: Debug>(f: T) {
    console::log_1(&JsValue::from_str(&format!("{:?}", f)))
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
