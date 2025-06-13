use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

/// Returns the current viewport height in pixels.
pub fn viewport_height() -> Result<f64, JsValue> {
    let webapp = webapp_object()?;
    let value = Reflect::get(&webapp, &JsValue::from_str("viewportHeight"))?;
    value
        .as_f64()
        .ok_or_else(|| JsValue::from_str("viewportHeight is not a number"))
}

/// Returns whether the Mini App is currently expanded.
pub fn is_expanded() -> Result<bool, JsValue> {
    let webapp = webapp_object()?;
    let value = Reflect::get(&webapp, &JsValue::from_str("isExpanded"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isExpanded is not a boolean"))
}

/// Calls Telegram.WebApp.expand() to expand the viewport.
pub fn expand_viewport() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let expand_fn = Reflect::get(&webapp, &JsValue::from_str("expand"))?;
    let func = expand_fn.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Registers a callback to be called on `viewportChanged` event.
///
/// # Safety
/// You must keep the closure alive manually if you want it to persist.
pub fn on_viewport_changed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let on_event = Reflect::get(&webapp, &JsValue::from_str("onEvent"))?;
    let func = on_event.dyn_into::<Function>()?;
    func.call2(
        &webapp,
        &JsValue::from_str("viewportChanged"),
        callback.as_ref()
    )?;
    Ok(())
}

/// Internal helper to get `Telegram.WebApp` JS object.
fn webapp_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &JsValue::from_str("Telegram"))?;
    js_sys::Reflect::get(&tg, &JsValue::from_str("WebApp"))
}
