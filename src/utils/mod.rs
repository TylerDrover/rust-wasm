use wasm_bindgen::prelude::*;


// Log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_f32(a: f32);
}


// Alert
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}