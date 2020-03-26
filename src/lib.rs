pub mod color;
pub mod design;
use color::NHPaletteItemPair;
use design::Design;

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document.body().expect("document should have a body");

    Ok(())
}

#[wasm_bindgen]
pub fn design_new() -> *mut Design {
    Box::into_raw(Box::new(Design::default()))
}

#[wasm_bindgen]
pub fn design_palette(design: *const Design) -> Result<JsValue, JsValue> {
    if let Some(design) = unsafe { design.as_ref() } {
        let items: Vec<NHPaletteItemPair> = design.palette().iter().map(|item| {
            let color: exoquant::Color = item.into();
            NHPaletteItemPair{
                item: item.clone(),
                rgba: format!("{:02X}{:02X}{:02X}{:02X}", color.r, color.g, color.b, color.a),
            }
        }).collect();

        JsValue::from_serde(&items).map_err(|e| e.to_string().into())
    } else {
        Err("Got a null pointer for design".into())
    }
}
