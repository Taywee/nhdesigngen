pub mod color;
pub mod design;
use design::Design;

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    Ok(())
}

#[wasm_bindgen]
pub fn design_new() -> *mut Design {
    Box::into_raw(Box::new(Design::default()))
}

#[wasm_bindgen]
pub fn design_print_palette(design: *const Design) -> String {
    if let Some(design) = unsafe { design.as_ref() } {
        format!("design palette: {:?}", design.palette())
    } else {
        panic!("AAAAAAHHHH");
    }
}
