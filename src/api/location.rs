use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

/// Calls Telegram.WebApp.requestLocation()
pub fn request_location() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func =
        Reflect::get(&webapp, &JsValue::from_str("requestLocation"))?.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Calls Telegram.WebApp.checkLocationAccess()
pub fn check_location_access() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func = Reflect::get(&webapp, &JsValue::from_str("checkLocationAccess"))?
        .dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Calls Telegram.WebApp.openLocationSettings()
pub fn open_location_settings() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func = Reflect::get(&webapp, &JsValue::from_str("openLocationSettings"))?
        .dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Registers a callback for `locationRequested` event
pub fn on_location_requested(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    add_event_listener("locationRequested", callback)
}

/// Registers a callback for `locationAllowed` event
pub fn on_location_allowed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    add_event_listener("locationAllowed", callback)
}

/// Internal helper to add JS event listeners
fn add_event_listener(event: &str, callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let on_event = Reflect::get(&webapp, &JsValue::from_str("onEvent"))?.dyn_into::<Function>()?;
    on_event.call2(&webapp, &JsValue::from_str(event), callback.as_ref())?;
    Ok(())
}

/// Internal helper to get Telegram.WebApp
fn webapp_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    Reflect::get(&tg, &JsValue::from_str("WebApp"))
}
