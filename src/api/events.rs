use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

/// Adds an event listener for Telegram.WebApp.onEvent(name, callback)
///
/// # Safety
/// You must keep the closure alive for as long as it's needed.
pub fn on_event(event_name: &str, callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let webapp = get_webapp_object()?;
    let func = Reflect::get(&webapp, &JsValue::from_str("onEvent"))?.dyn_into::<Function>()?;
    func.call2(&webapp, &JsValue::from_str(event_name), callback.as_ref())?;
    Ok(())
}

/// Removes a previously registered event listener.
///
/// This is optional but recommended for cleanup.
pub fn off_event(event_name: &str, callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let webapp = get_webapp_object()?;
    let func = Reflect::get(&webapp, &JsValue::from_str("offEvent"))?.dyn_into::<Function>()?;
    func.call2(&webapp, &JsValue::from_str(event_name), callback.as_ref())?;
    Ok(())
}

/// Internal helper to get `Telegram.WebApp` JS object.
fn get_webapp_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let telegram = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    Reflect::get(&telegram, &JsValue::from_str("WebApp"))
}
