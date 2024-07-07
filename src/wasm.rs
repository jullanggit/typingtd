use bevy::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlCanvasElement};
// Plugin
pub struct WasmPlugin;
impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fit_canvas_to_parent);
    }
}

#[expect(clippy::let_underscore_must_use)]
fn fit_canvas_to_parent() {
    let canvas: HtmlCanvasElement = web_sys::window()
        .expect("Window should exist")
        .document()
        .expect("Window should have document field")
        .query_selector("canvas")
        .expect("Document filed should have canvas")
        .expect("Document filed should have canvas")
        .unchecked_into();
    let style = canvas.style();
    let _ = style.set_property("width", "100%");
    let _ = style.set_property("height", "100%");
}
