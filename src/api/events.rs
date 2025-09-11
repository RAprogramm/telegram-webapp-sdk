use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
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

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsValue, closure::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().expect("window should be available");
        let telegram = Object::new();
        let webapp = Object::new();

        // onEvent stores the callback under the event name.
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        // offEvent removes the stored callback.
        let off_event = Function::new_with_args("name", "delete this[name];");

        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn registers_and_removes_callback() {
        let webapp = setup_webapp();
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);

        on_event("test-event", &cb).expect("register callback");
        let has = Reflect::has(&webapp, &JsValue::from_str("test-event")).unwrap();
        assert!(has, "callback was not stored");

        off_event("test-event", &cb).expect("remove callback");
        let has_after = Reflect::has(&webapp, &JsValue::from_str("test-event")).unwrap();
        assert!(!has_after, "callback was not removed");
    }
}
