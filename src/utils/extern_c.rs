use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = SoundManager)]
    pub fn play(sound_name: &str);

    #[wasm_bindgen(js_namespace = window )]
    pub fn get_random(min: f64, max: f64) -> f64;

    #[wasm_bindgen(js_namespace = window )]
    pub fn get_random_int(min: u32, max: u32) -> u32;

    #[wasm_bindgen(js_namespace = window )]
    pub fn is_game_paused() -> bool;
    
    #[wasm_bindgen(js_namespace = window )]
    pub fn screen_size() -> u32;
}
