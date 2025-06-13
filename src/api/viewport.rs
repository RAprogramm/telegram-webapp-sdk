use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

use crate::logger::{debug, warn};

/// Returns the current viewport height in pixels.
pub fn get_viewport_height() -> Option<f64> {
    let webapp = webapp_object().ok()?;
    let value = Reflect::get(&webapp, &"viewportHeight".into()).ok()?;
    let result = value.as_f64();
    if let Some(px) = result {
        debug(&format!("viewportHeight: {}px", px));
    } else {
        warn("viewportHeight is not a number");
    }
    result
}

/// Returns whether the Mini App is currently expanded.
pub fn get_is_expanded() -> Option<bool> {
    let webapp = webapp_object().ok()?;
    let value = Reflect::get(&webapp, &"isExpanded".into()).ok()?;
    let result = value.as_bool();
    if let Some(exp) = result {
        debug(&format!("isExpanded: {}", exp));
    } else {
        warn("isExpanded is not a boolean");
    }
    result
}

/// Calls Telegram.WebApp.expand() to expand the viewport.
pub fn expand_viewport() {
    if let Ok(webapp) = webapp_object() {
        let _ = Reflect::get(&webapp, &"expand".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| f.call0(&webapp).ok());
        debug("Called WebApp.expand()");
    } else {
        warn("Cannot expand viewport: WebApp not found");
    }
}

/// Registers a callback to be called on `viewportChanged` event.
///
/// ⚠️ Closure must be kept alive outside.
pub fn on_viewport_changed(callback: &Closure<dyn Fn()>) {
    if let Ok(webapp) = webapp_object() {
        let _ = Reflect::get(&webapp, &"onEvent".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| {
                f.call2(&webapp, &"viewportChanged".into(), callback.as_ref())
                    .ok()
            });
        debug("Registered viewportChanged event handler");
    } else {
        warn("Cannot register viewportChanged: WebApp not found");
    }
}

/// Internal helper to get `Telegram.WebApp` JS object.
fn webapp_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &"Telegram".into())?;
    Reflect::get(&tg, &"WebApp".into())
}
