use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
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
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar").await?;
/// # Ok(()) }
/// ```
pub async fn set(key: &str, value: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("set"))?.dyn_into::<Function>()?;
    let promise = func
        .call2(&storage, &JsValue::from_str(key), &JsValue::from_str(value))?
        .dyn_into::<Promise>()?;
    JsFuture::from(promise).await?;
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
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar").await?;
/// let value = get("foo").await?;
/// assert_eq!(value.as_deref(), Some("bar"));
/// # Ok(()) }
/// ```
pub async fn get(key: &str) -> Result<Option<String>, JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("get"))?.dyn_into::<Function>()?;
    let promise = func
        .call1(&storage, &JsValue::from_str(key))?
        .dyn_into::<Promise>()?;
    let value = JsFuture::from(promise).await?;
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
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar").await?;
/// remove("foo").await?;
/// # Ok(()) }
/// ```
pub async fn remove(key: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("remove"))?.dyn_into::<Function>()?;
    let promise = func
        .call1(&storage, &JsValue::from_str(key))?
        .dyn_into::<Promise>()?;
    JsFuture::from(promise).await?;
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
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set("foo", "bar").await?;
/// clear().await?;
/// # Ok(()) }
/// ```
pub async fn clear() -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("clear"))?.dyn_into::<Function>()?;
    let promise = func.call0(&storage)?.dyn_into::<Promise>()?;
    JsFuture::from(promise).await?;
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

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k,v", "this[k] = v; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"set".into(), &func);
        assert!(set("a", "b").await.is_ok());
        let val = Reflect::get(&storage, &"a".into()).unwrap();
        assert_eq!(val.as_string().as_deref(), Some("b"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_err() {
        assert!(set("a", "b").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k", "return this[k];");
        let _ = Reflect::set(&storage, &"get".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        let value = get("a").await.unwrap();
        assert_eq!(value.as_deref(), Some("b"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_err() {
        assert!(get("a").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn remove_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_with_args("k", "delete this[k]; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"remove".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(remove("a").await.is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn remove_err() {
        assert!(remove("a").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_calls_js() {
        let storage = setup_device_storage();
        let func = Function::new_no_args(
            "Object.keys(this).forEach(k => delete this[k]); return Promise.resolve();"
        );
        let _ = Reflect::set(&storage, &"clear".into(), &func);
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(clear().await.is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_err() {
        assert!(clear().await.is_err());
    }
}
