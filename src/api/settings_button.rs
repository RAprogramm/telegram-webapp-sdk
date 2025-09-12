use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Show the Telegram Settings Button.
///
/// # Errors
/// Returns `Err` if the underlying JavaScript call fails or the button is
/// missing.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::settings_button::show;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// show()?;
/// # Ok(()) }
/// ```
pub fn show() -> Result<(), JsValue> {
    let button = settings_button_object()?;
    let func = Reflect::get(&button, &"show".into())?.dyn_into::<Function>()?;
    func.call0(&button)?;
    Ok(())
}

/// Hide the Telegram Settings Button.
///
/// # Errors
/// Returns `Err` if the underlying JavaScript call fails or the button is
/// missing.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::settings_button::hide;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// hide()?;
/// # Ok(()) }
/// ```
pub fn hide() -> Result<(), JsValue> {
    let button = settings_button_object()?;
    let func = Reflect::get(&button, &"hide".into())?.dyn_into::<Function>()?;
    func.call0(&button)?;
    Ok(())
}

/// Register a callback for Settings Button clicks.
///
/// # Safety
/// The closure must be kept alive for as long as it's registered.
///
/// # Errors
/// Returns `Err` if the registration fails or the button is missing.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::settings_button::on_click;
/// use wasm_bindgen::prelude::Closure;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
/// on_click(&cb)?;
/// # Ok(()) }
/// ```
pub fn on_click(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let button = settings_button_object()?;
    let func = Reflect::get(&button, &"onClick".into())?.dyn_into::<Function>()?;
    func.call1(&button, callback.as_ref())?;
    Ok(())
}

/// Remove a previously registered click callback.
///
/// # Errors
/// Returns `Err` if the deregistration fails or the button is missing.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::settings_button::{off_click, on_click};
/// use wasm_bindgen::prelude::Closure;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
/// on_click(&cb)?;
/// off_click(&cb)?;
/// # Ok(()) }
/// ```
pub fn off_click(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    let button = settings_button_object()?;
    let func = Reflect::get(&button, &"offClick".into())?.dyn_into::<Function>()?;
    func.call1(&button, callback.as_ref())?;
    Ok(())
}

fn settings_button_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &"Telegram".into())?;
    let webapp = Reflect::get(&tg, &"WebApp".into())?;
    Reflect::get(&webapp, &"SettingsButton".into())
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_button() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let button = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"SettingsButton".into(), &button);
        button
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_calls_js() {
        let button = setup_button();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&button, &"show".into(), &func);
        assert!(show().is_ok());
        assert!(
            Reflect::get(&button, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_calls_js() {
        let button = setup_button();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&button, &"hide".into(), &func);
        assert!(hide().is_ok());
        assert!(
            Reflect::get(&button, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn click_callbacks() {
        let button = setup_button();
        let on = Function::new_with_args("cb", "this.cb = cb;");
        let off = Function::new_with_args("cb", "delete this.cb;");
        let _ = Reflect::set(&button, &"onClick".into(), &on);
        let _ = Reflect::set(&button, &"offClick".into(), &off);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_click(&cb).expect("on");
        assert!(Reflect::has(&button, &"cb".into()).unwrap());
        off_click(&cb).expect("off");
        assert!(!Reflect::has(&button, &"cb".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_err() {
        let _ = setup_button();
        assert!(show().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn on_click_err() {
        let _ = setup_button();
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        assert!(on_click(&cb).is_err());
    }
}
