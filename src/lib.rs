pub mod color;
pub mod design;
use color::HSVRGBA;
use design::Design;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
        let items: Vec<HSVRGBA> = design.palette().iter().map(Into::into).collect();

        JsValue::from_serde(&items).map_err(|e| e.to_string().into())
    } else {
        Err("Got a null pointer for design".into())
    }
}

#[wasm_bindgen]
pub fn design_load_palette(design: *mut Design, buffers: JsValue) -> Result<(), JsValue> {
    if let Some(design) = unsafe { design.as_mut() } {
        let data: Vec<Vec<(u8, u8, u8, u8)>> = buffers.into_serde().map_err(|e| JsValue::from(e.to_string()))?;
        design.load_histogram_from_buffers(data);
        Ok(())
    } else {
        Err("Got a null pointer for design".into())
    }
}

#[wasm_bindgen]
pub fn design_optimize_palette(design: *mut Design, optimizer: String) -> Result<(), JsValue> {
    if let Some(design) = unsafe { design.as_mut() } {
        match optimizer.as_str() {
            "kmeans" => design.optimize_palette(exoquant::optimizer::KMeans),
            "weightedkmeans" => design.optimize_palette(exoquant::optimizer::WeightedKMeans),
            e => return Err(format!("optimizer {} not recognized", e).into()),
        }
        Ok(())
    } else {
        Err("Got a null pointer for design".into())
    }
}
