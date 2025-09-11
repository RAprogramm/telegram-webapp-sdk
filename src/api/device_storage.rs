use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

/// Stores a value under the given key in Telegram's device storage.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `deviceStorage` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::set;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar")?;
/// # Ok(()) }
/// ```
pub fn set(key: &str, value: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("set"))?.dyn_into::<Function>()?;
    func.call2(&storage, &JsValue::from_str(key), &JsValue::from_str(value))?;
    Ok(())
}

/// Retrieves a value from Telegram's device storage.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `deviceStorage` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{get, set};
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar")?;
/// let value = get("foo")?;
/// assert_eq!(value.as_deref(), Some("bar"));
/// # Ok(()) }
/// ```
pub fn get(key: &str) -> Result<Option<String>, JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("get"))?.dyn_into::<Function>()?;
    let value = func.call1(&storage, &JsValue::from_str(key))?;
    Ok(value.as_string())
}

/// Removes a value from Telegram's device storage.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `deviceStorage` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{remove, set};
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar")?;
/// remove("foo")?;
/// # Ok(()) }
/// ```
pub fn remove(key: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("remove"))?.dyn_into::<Function>()?;
    func.call1(&storage, &JsValue::from_str(key))?;
    Ok(())
}

/// Clears all entries from Telegram's device storage.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `deviceStorage` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{clear, set};
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar")?;
/// clear()?;
/// # Ok(()) }
/// ```
pub fn clear() -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("clear"))?.dyn_into::<Function>()?;
    func.call0(&storage)?;
    Ok(())
}

fn device_storage_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("deviceStorage"))
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_device_storage() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let storage = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"deviceStorage".into(), &storage);
        storage
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn set_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k,v", "this[k] = v;");
        let _ = Reflect::set(&storage, &"set".into(), &func);
        assert!(set("a", "b").is_ok());
        let val = Reflect::get(&storage, &"a".into()).unwrap();
        assert_eq!(val.as_string().as_deref(), Some("b"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn set_err() {
        assert!(set("a", "b").is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn get_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k", "return this[k];");
        let _ = Reflect::set(&storage, &"get".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        let value = get("a").unwrap();
        assert_eq!(value.as_deref(), Some("b"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn get_err() {
        assert!(get("a").is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn remove_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k", "delete this[k];");
        let _ = Reflect::set(&storage, &"remove".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(remove("a").is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn remove_err() {
        assert!(remove("a").is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn clear_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_no_args("Object.keys(this).forEach(k => delete this[k]);");
        let _ = Reflect::set(&storage, &"clear".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(clear().is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn clear_err() {
        assert!(clear().is_err());
    }
}
