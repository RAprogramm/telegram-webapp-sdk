use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
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

/// Returns the current viewport width in pixels.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::viewport::get_viewport_width;
/// let _ = get_viewport_width();
/// ```
pub fn get_viewport_width() -> Option<f64> {
    let webapp = webapp_object().ok()?;
    let value = Reflect::get(&webapp, &"viewportWidth".into()).ok()?;
    let result = value.as_f64();
    if let Some(px) = result {
        debug(&format!("viewportWidth: {}px", px));
    } else {
        warn("viewportWidth is not a number");
    }
    result
}

/// Returns the stable viewport height in pixels.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::viewport::get_viewport_stable_height;
/// let _ = get_viewport_stable_height();
/// ```
pub fn get_viewport_stable_height() -> Option<f64> {
    let webapp = webapp_object().ok()?;
    let value = Reflect::get(&webapp, &"viewportStableHeight".into()).ok()?;
    let result = value.as_f64();
    if let Some(px) = result {
        debug(&format!("viewportStableHeight: {}px", px));
    } else {
        warn("viewportStableHeight is not a number");
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

/// Calls `Telegram.WebApp.expand()` to expand the viewport.
///
/// # Errors
/// Returns [`JsValue`] if the underlying JS call fails.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::viewport::expand_viewport;
/// let _ = expand_viewport();
/// ```
pub fn expand_viewport() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func = Reflect::get(&webapp, &"expand".into())?.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    debug("Called WebApp.expand()");
    Ok(())
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

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn width_and_stable_height() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportWidth".into(), &JsValue::from_f64(200.0));
        let _ = Reflect::set(
            &webapp,
            &"viewportStableHeight".into(),
            &JsValue::from_f64(500.0)
        );
        assert_eq!(get_viewport_width(), Some(200.0));
        assert_eq!(get_viewport_stable_height(), Some(500.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn expand_viewport_success() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this._expanded = true;");
        let _ = Reflect::set(&webapp, &"expand".into(), &func);
        assert!(expand_viewport().is_ok());
        let called = Reflect::get(&webapp, &"_expanded".into()).unwrap();
        assert_eq!(called.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn expand_viewport_failure() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"expand".into(), &JsValue::from_f64(1.0));
        assert!(expand_viewport().is_err());
    }
}
