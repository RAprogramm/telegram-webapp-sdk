use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Initializes `Telegram.WebApp.locationManager`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `locationManager` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::location_manager::init;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// init()?;
/// # Ok(()) }
/// ```
pub fn init() -> Result<(), JsValue> {
    let manager = location_manager_object()?;
    let func = Reflect::get(&manager, &JsValue::from_str("init"))?.dyn_into::<Function>()?;
    func.call0(&manager)?;
    Ok(())
}

/// Retrieves the current location via `getLocation`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `locationManager` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::location_manager::get_location;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let _loc = get_location()?;
/// # Ok(()) }
/// ```
pub fn get_location() -> Result<JsValue, JsValue> {
    let manager = location_manager_object()?;
    let func =
        Reflect::get(&manager, &JsValue::from_str("getLocation"))?.dyn_into::<Function>()?;
    func.call0(&manager)
}

/// Opens the location settings via `openSettings`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `locationManager` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::location_manager::open_settings;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// open_settings()?;
/// # Ok(()) }
/// ```
pub fn open_settings() -> Result<(), JsValue> {
    let manager = location_manager_object()?;
    let func =
        Reflect::get(&manager, &JsValue::from_str("openSettings"))?.dyn_into::<Function>()?;
    func.call0(&manager)?;
    Ok(())
}

/// Registers a callback for `locationManagerUpdated` events.
///
/// # Errors
/// Returns `Err(JsValue)` if the event registration fails or `WebApp` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::location_manager::on_location_manager_updated;
/// use wasm_bindgen::closure::Closure;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
/// on_location_manager_updated(&cb)?;
/// cb.forget();
/// # Ok(()) }
/// ```
pub fn on_location_manager_updated(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    add_event_listener("locationManagerUpdated", callback)
}

/// Registers a callback for `locationRequested` events.
///
/// # Errors
/// Returns `Err(JsValue)` if the event registration fails or `WebApp` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::location_manager::on_location_requested;
/// use wasm_bindgen::closure::Closure;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
/// on_location_requested(&cb)?;
/// cb.forget();
/// # Ok(()) }
/// ```
pub fn on_location_requested(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    add_event_listener("locationRequested", callback)
}

fn add_event_listener(event: &str, callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let on_event = Reflect::get(&webapp, &JsValue::from_str("onEvent"))?.dyn_into::<Function>()?;
    on_event.call2(&webapp, &JsValue::from_str(event), callback.as_ref())?;
    Ok(())
}

fn location_manager_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("locationManager"))
}

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
    fn setup_location_manager() -> (Object, Object) {
        let win = window().expect("window should be available");
        let telegram = Object::new();
        let webapp = Object::new();
        let manager = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"locationManager".into(), &manager);
        (webapp, manager)
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn init_ok() {
        let (_webapp, manager) = setup_location_manager();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&manager, &"init".into(), &func);
        assert!(init().is_ok());
        assert!(
            Reflect::get(&manager, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn init_err() {
        let _ = setup_location_manager();
        assert!(init().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn get_location_ok() {
        let (_webapp, manager) = setup_location_manager();
        let location = Object::new();
        let func = Function::new_no_args("return this.loc;");
        let _ = Reflect::set(&manager, &"getLocation".into(), &func);
        let _ = Reflect::set(&manager, &"loc".into(), &location);
        let result = get_location().expect("location");
        assert!(result.is_object());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn get_location_err() {
        let _ = setup_location_manager();
        assert!(get_location().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_settings_ok() {
        let (_webapp, manager) = setup_location_manager();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&manager, &"openSettings".into(), &func);
        assert!(open_settings().is_ok());
        assert!(
            Reflect::get(&manager, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_settings_err() {
        let _ = setup_location_manager();
        assert!(open_settings().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn registers_location_manager_updated_callback() {
        let (webapp, _manager) = setup_location_manager();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_location_manager_updated(&cb).expect("register callback");
        let has = Reflect::has(&webapp, &JsValue::from_str("locationManagerUpdated")).unwrap();
        assert!(has);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn registers_location_requested_callback() {
        let (webapp, _manager) = setup_location_manager();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_location_requested(&cb).expect("register callback");
        let has = Reflect::has(&webapp, &JsValue::from_str("locationRequested")).unwrap();
        assert!(has);
    }
}
