use crate::{ PLAYER, ThreadSafeImage, SpriteSheet, HashMap, wasm_bindgen, Player, CanvasRenderingContext2d, HtmlCanvasElement, JsValue };
use wasm_bindgen::JsCast;
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

