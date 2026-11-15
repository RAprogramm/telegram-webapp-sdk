// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Access to Telegram's secure storage (`Telegram.WebApp.SecureStorage`).
//!
//! The bridge exposes callback-style methods (`setItem`, `getItem`,
//! `restoreItem`, `removeItem`, `clear`); this module wraps them into Rust
//! futures.

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::window;

use crate::utils::promise::{await_one_shot, one_shot_promise};

/// Value read from Telegram's secure storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecureStorageValue {
    /// Stored value, absent when the key is not stored on the device.
    pub value:       Option<String>,
    /// Whether the key can be restored from the current device.
    pub can_restore: bool
}

/// Stores a value under the given key in Telegram's secure storage.
///
/// Values are stored in an encrypted form and can be restored after the user
/// reinstalls the application.
///
/// Mirrors `SecureStorage.setItem(key, value, callback)`: the promise
/// resolves once the client confirms the write and rejects with the client
/// error otherwise.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `SecureStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::secure_storage::set_item;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("token", "123").await?;
/// # Ok(()) }
/// ```
pub async fn set_item(key: &str, value: &str) -> Result<(), JsValue> {
    let storage = secure_storage_object()?;
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

/// Retrieves a value from Telegram's secure storage.
///
/// Mirrors `SecureStorage.getItem(key, callback)`. A key that is not stored
/// resolves to an empty [`SecureStorageValue`], whose `can_restore` flag
/// reports whether the client can restore the key from the current device.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `SecureStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::secure_storage::{get_item, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("token", "123").await?;
/// let stored = get_item("token").await?;
/// assert_eq!(stored.value.as_deref(), Some("123"));
/// # Ok(()) }
/// ```
pub async fn get_item(key: &str) -> Result<SecureStorageValue, JsValue> {
    let storage = secure_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(
            move |error: JsValue, value: JsValue, can_restore: JsValue| {
                if error.is_null() || error.is_undefined() {
                    let payload = Object::new();
                    let _ = Reflect::set(&payload, &"value".into(), &value);
                    let _ = Reflect::set(&payload, &"can_restore".into(), &can_restore);
                    let _ = resolve.call1(&JsValue::NULL, &payload.into());
                } else {
                    let _ = reject.call1(&JsValue::NULL, &error);
                }
            }
        );
        let method = storage_method(&storage, "getItem")?;
        method.call2(&storage, &JsValue::from_str(key), &callback)?;
        Ok(())
    });
    let payload = await_one_shot(promise).await?;
    let value = Reflect::get(&payload, &"value".into())?.as_string();
    let can_restore = Reflect::get(&payload, &"can_restore".into())?
        .as_bool()
        .unwrap_or(false);
    Ok(SecureStorageValue {
        value,
        can_restore
    })
}

/// Restores a previously removed value from Telegram's secure storage.
///
/// Mirrors `SecureStorage.restoreItem(key, callback)`: the client asks the
/// user for permission, and the promise resolves with the restored value or
/// `None` when the user declines.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `SecureStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::secure_storage::{remove_item, restore_item, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("token", "123").await?;
/// remove_item("token").await?;
/// let _ = restore_item("token").await?;
/// # Ok(()) }
/// ```
pub async fn restore_item(key: &str) -> Result<Option<String>, JsValue> {
    let storage = secure_storage_object()?;
    let promise = one_shot_promise(move |resolve, reject| {
        let callback = Closure::once_into_js(move |error: JsValue, value: JsValue| {
            settle(&resolve, &reject, &error, &value);
        });
        let method = storage_method(&storage, "restoreItem")?;
        method.call2(&storage, &JsValue::from_str(key), &callback)?;
        Ok(())
    });
    let value = await_one_shot(promise).await?;
    Ok(value.as_string())
}

/// Removes a value from Telegram's secure storage.
///
/// Mirrors `SecureStorage.removeItem(key, callback)`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `SecureStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::secure_storage::{remove_item, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("token", "123").await?;
/// remove_item("token").await?;
/// # Ok(()) }
/// ```
pub async fn remove_item(key: &str) -> Result<(), JsValue> {
    let storage = secure_storage_object()?;
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

/// Clears all entries from Telegram's secure storage.
///
/// Mirrors `SecureStorage.clear(callback)`.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails, if `SecureStorage`
/// is missing, or if the client reports a storage error.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::secure_storage::{clear, set_item};
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// set_item("token", "123").await?;
/// clear().await?;
/// # Ok(()) }
/// ```
pub async fn clear() -> Result<(), JsValue> {
    let storage = secure_storage_object()?;
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

fn secure_storage_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("SecureStorage"))
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_secure_storage() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let storage = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"SecureStorage".into(), &storage);
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
                "cb(null, typeof this[k] === 'undefined' ? null : this[k], false);"
            )
        );
        let _ = Reflect::set(
            &storage,
            &"restoreItem".into(),
            &Function::new_with_args("k,cb", "cb(null, this[k] === undefined ? null : this[k]);")
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
        let storage = setup_secure_storage();
        assert!(set_item("token", "123").await.is_ok());
        let val = Reflect::get(&storage, &"token".into()).unwrap();
        assert_eq!(val.as_string().as_deref(), Some("123"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_item_reports_client_error() {
        let storage = setup_secure_storage();
        let _ = Reflect::set(
            &storage,
            &"setItem".into(),
            &Function::new_with_args("k,v,cb", "cb('storage failure', false);")
        );
        assert!(set_item("token", "123").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn set_item_without_storage_fails() {
        remove_telegram();
        assert!(set_item("token", "123").await.is_err());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_item_calls_js() {
        let storage = setup_secure_storage();
        let _ = Reflect::set(&storage, &"token".into(), &JsValue::from_str("123"));
        let stored = get_item("token").await.unwrap();
        assert_eq!(stored.value.as_deref(), Some("123"));
        assert!(!stored.can_restore);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn get_item_returns_empty_value_for_missing_key() {
        setup_secure_storage();
        let stored = get_item("missing").await.unwrap();
        assert!(stored.value.is_none());
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn restore_item_calls_js() {
        let storage = setup_secure_storage();
        let _ = Reflect::set(&storage, &"token".into(), &JsValue::from_str("123"));
        let restored = restore_item("token").await.unwrap();
        assert_eq!(restored.as_deref(), Some("123"));
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn remove_item_calls_js() {
        let storage = setup_secure_storage();
        let _ = Reflect::set(&storage, &"token".into(), &JsValue::from_str("123"));
        assert!(remove_item("token").await.is_ok());
        let has = Reflect::has(&storage, &"token".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_calls_js() {
        let storage = setup_secure_storage();
        let _ = Reflect::set(&storage, &"token".into(), &JsValue::from_str("123"));
        assert!(clear().await.is_ok());
        let has = Reflect::has(&storage, &"token".into()).unwrap();
        assert!(!has);
    }

    #[wasm_bindgen_test(async)]
    #[allow(dead_code)]
    async fn clear_without_storage_fails() {
        remove_telegram();
        assert!(clear().await.is_err());
    }
}
