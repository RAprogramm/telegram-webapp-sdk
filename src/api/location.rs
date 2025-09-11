use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
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
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_location_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"requestLocation".into(), &func);
        assert!(request_location().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_location_err() {
        let _ = setup_webapp();
        assert!(request_location().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn check_location_access_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"checkLocationAccess".into(), &func);
        assert!(check_location_access().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn check_location_access_err() {
        let _ = setup_webapp();
        assert!(check_location_access().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_location_settings_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"openLocationSettings".into(), &func);
        assert!(open_location_settings().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_location_settings_err() {
        let _ = setup_webapp();
        assert!(open_location_settings().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn registers_location_requested_callback() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_location_requested(&cb).expect("register callback");
        let has = Reflect::has(&webapp, &JsValue::from_str("locationRequested")).unwrap();
        assert!(has);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn registers_location_allowed_callback() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_location_allowed(&cb).expect("register callback");
        let has = Reflect::has(&webapp, &JsValue::from_str("locationAllowed")).unwrap();
        assert!(has);
    }
}
