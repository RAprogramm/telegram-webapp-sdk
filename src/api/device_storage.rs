// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Access to Telegram's device storage (`Telegram.WebApp.DeviceStorage`).
//!
//! The bridge exposes callback-style methods (`setItem`, `getItem`,
//! `removeItem`, `clear`); this module wraps them into Rust futures.

use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::utils::promise::{await_one_shot, one_shot_promise};

/// Stores a value under the given key in Telegram's device storage.
///
/// Mirrors `DeviceStorage.setItem(key, value, callback)`: the promise
/// resolves once the client confirms the write and rejects with the client
/// error otherwise.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `DeviceStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::set_item;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("foo", "bar").await?;
/// # Ok(()) }
/// ```
pub async fn set_item(key: &str, value: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(move |error: JsValue, _stored: JsValue| {
            settle(&resolve, &reject, &error, &JsValue::UNDEFINED);
        });
        let method = storage_method(&storage, "setItem")?;
        method.call3(
            &storage,
            &JsValue::from_str(key),
            &JsValue::from_str(value),
            &callback
        )?;
        Ok(())
    });
    await_one_shot(promise).await?;
    Ok(())
}

/// Retrieves a value from Telegram's device storage.
///
/// Mirrors `DeviceStorage.getItem(key, callback)`. A key that is not stored
/// resolves to `None`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `DeviceStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{get_item, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("foo", "bar").await?;
/// let value = get_item("foo").await?;
/// assert_eq!(value.as_deref(), Some("bar"));
/// # Ok(()) }
/// ```
pub async fn get_item(key: &str) -> Result<Option<String>, JsValue> {
    let storage = device_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(move |error: JsValue, value: JsValue| {
            settle(&resolve, &reject, &error, &value);
        });
        let method = storage_method(&storage, "getItem")?;
        method.call2(&storage, &JsValue::from_str(key), &callback)?;
        Ok(())
    });
    let value = await_one_shot(promise).await?;
    Ok(value.as_string())
}

/// Removes a value from Telegram's device storage.
///
/// Mirrors `DeviceStorage.removeItem(key, callback)`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `DeviceStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{remove_item, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("foo", "bar").await?;
/// remove_item("foo").await?;
/// # Ok(()) }
/// ```
pub async fn remove_item(key: &str) -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(move |error: JsValue, _removed: JsValue| {
            settle(&resolve, &reject, &error, &JsValue::UNDEFINED);
        });
        let method = storage_method(&storage, "removeItem")?;
        method.call2(&storage, &JsValue::from_str(key), &callback)?;
        Ok(())
    });
    await_one_shot(promise).await?;
    Ok(())
}

/// Clears all entries from Telegram's device storage.
///
/// Mirrors `DeviceStorage.clear(callback)`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `DeviceStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::device_storage::{clear, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("foo", "bar").await?;
/// clear().await?;
/// # Ok(()) }
/// ```
pub async fn clear() -> Result<(), JsValue> {
    let storage = device_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(move |error: JsValue, _cleared: JsValue| {
            settle(&resolve, &reject, &error, &JsValue::UNDEFINED);
        });
        let method = storage_method(&storage, "clear")?;
        method.call1(&storage, &callback)?;
        Ok(())
    });
    await_one_shot(promise).await?;
    Ok(())
}

/// Resolves the promise when the client reported no error and rejects it
/// with the client error otherwise.
fn settle(resolve: &Function, reject: &Function, error: &JsValue, value: &JsValue) {
    if error.is_null() || error.is_undefined() {
        let _ = resolve.call1(&JsValue::NULL, value);
    } else {
        let _ = reject.call1(&JsValue::NULL, error);
    }
}

fn storage_method(storage: &JsValue, name: &str) -> Result<Function, JsValue> {
    let value = Reflect::get(storage, &JsValue::from_str(name))?;
    value
        .dyn_into::<Function>()
        .map_err(|_| JsValue::from_str(&format!("{name} is not a function")))
}

fn device_storage_object() -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("DeviceStorage"))
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_device_storage(script: &str) -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let storage = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"DeviceStorage".into(), &storage);
        let _ = Reflect::set(
            &storage,
            &"setItem".into(),
            &Function::new_with_args("k,v,cb", "this[k] = v; cb(null, true);")
        );
        let _ = Reflect::set(
            &storage,
            &"getItem".into(),
            &Function::new_with_args(
                "k,cb",
                "cb(null, typeof this[k] === 'undefined' ? null : this[k]);"
            )
        );
        let _ = Reflect::set(
            &storage,
            &"removeItem".into(),
            &Function::new_with_args("k,cb", "delete this[k]; cb(null, true);")
        );
        let _ = Reflect::set(
            &storage,
            &"clear".into(),
            &Function::new_with_args(
                "cb",
                "Object.keys(this).forEach(k => delete this[k]); cb(null, true);"
            )
        );
        if !script.is_empty() {
            let _ = Reflect::set(
                &storage,
                &"setItem".into(),
                &Function::new_with_args("k,v,cb", script)
            );
        }
        storage
    }

    #[allow(dead_code)]
    fn remove_telegram() {
        let win = window().unwrap();
        let _ = Reflect::delete_property(&win, &"Telegram".into());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_item_calls_js() {
        let storage = setup_device_storage("");
        assert!(set_item("a", "b").await.is_ok());
        let val = Reflect::get(&storage, &"a".into()).unwrap();
        assert_eq!(val.as_string().as_deref(), Some("b"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_item_reports_client_error() {
        setup_device_storage("cb('storage failure', false);");
        assert!(set_item("a", "b").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_item_without_storage_fails() {
        remove_telegram();
        assert!(set_item("a", "b").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_item_calls_js() {
        let storage = setup_device_storage("");
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        let value = get_item("a").await.unwrap();
        assert_eq!(value.as_deref(), Some("b"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_item_returns_none_for_missing_key() {
        setup_device_storage("");
        let value = get_item("missing").await.unwrap();
        assert!(value.is_none());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_item_reports_client_error() {
        let storage = setup_device_storage("");
        let _ = Reflect::set(
            &storage,
            &"getItem".into(),
            &Function::new_with_args("k,cb", "cb('storage failure', null);")
        );
        assert!(get_item("a").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn remove_item_calls_js() {
        let storage = setup_device_storage("");
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(remove_item("a").await.is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_calls_js() {
        let storage = setup_device_storage("");
        let _ = Reflect::set(&storage, &"a".into(), &JsValue::from_str("b"));
        assert!(clear().await.is_ok());
        let has = Reflect::has(&storage, &"a".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_without_storage_fails() {
        remove_telegram();
        assert!(clear().await.is_err());
    }
}
