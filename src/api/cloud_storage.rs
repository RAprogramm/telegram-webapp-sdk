use js_sys::{Array, Function, Promise, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Returns the `Telegram.WebApp.CloudStorage` object.
fn cloud_storage_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("CloudStorage"))
}

/// Calls `Telegram.WebApp.CloudStorage.getItem()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::get_item;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let value = JsFuture::from(get_item("key")?).await?;
/// # Ok(())
/// # }
/// ```
pub fn get_item(key: &str) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("getItem"))?.dyn_into::<Function>()?;
    func.call1(&storage, &JsValue::from_str(key))?
        .dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.setItem()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::set_item;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// JsFuture::from(set_item("key", "value")?).await?;
/// # Ok(())
/// # }
/// ```
pub fn set_item(key: &str, value: &str) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("setItem"))?.dyn_into::<Function>()?;
    func.call2(&storage, &JsValue::from_str(key), &JsValue::from_str(value))?
        .dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.removeItem()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::remove_item;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// JsFuture::from(remove_item("key")?).await?;
/// # Ok(())
/// # }
/// ```
pub fn remove_item(key: &str) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("removeItem"))?.dyn_into::<Function>()?;
    func.call1(&storage, &JsValue::from_str(key))?
        .dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.getItems()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::get_items;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let _ = JsFuture::from(get_items(&["a", "b"])?).await?;
/// # Ok(())
/// # }
/// ```
pub fn get_items(keys: &[&str]) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("getItems"))?.dyn_into::<Function>()?;
    let array = Array::new();
    for key in keys {
        array.push(&JsValue::from_str(key));
    }
    func.call1(&storage, &array.into())?.dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.setItems()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::set_items;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// JsFuture::from(set_items(&[("a", "1"), ("b", "2")])?).await?;
/// # Ok(())
/// # }
/// ```
pub fn set_items(items: &[(&str, &str)]) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("setItems"))?.dyn_into::<Function>()?;
    let obj = js_sys::Object::new();
    for (key, value) in items {
        Reflect::set(&obj, &JsValue::from_str(key), &JsValue::from_str(value))?;
    }
    func.call1(&storage, &obj.into())?.dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.removeItems()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::remove_items;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// JsFuture::from(remove_items(&["a", "b"])?).await?;
/// # Ok(())
/// # }
/// ```
pub fn remove_items(keys: &[&str]) -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func =
        Reflect::get(&storage, &JsValue::from_str("removeItems"))?.dyn_into::<Function>()?;
    let array = Array::new();
    for key in keys {
        array.push(&JsValue::from_str(key));
    }
    func.call1(&storage, &array.into())?.dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.getKeys()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::get_keys;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// let _ = JsFuture::from(get_keys()?).await?;
/// # Ok(())
/// # }
/// ```
pub fn get_keys() -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("getKeys"))?.dyn_into::<Function>()?;
    func.call0(&storage)?.dyn_into::<Promise>()
}

/// Calls `Telegram.WebApp.CloudStorage.clear()`.
///
/// # Errors
/// Returns `Err(JsValue)` if CloudStorage or the method is unavailable, or if
/// the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::cloud_storage::clear;
/// use wasm_bindgen_futures::JsFuture;
/// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
/// JsFuture::from(clear()?).await?;
/// # Ok(())
/// # }
/// ```
pub fn clear() -> Result<Promise, JsValue> {
    let storage = cloud_storage_object()?;
    let func = Reflect::get(&storage, &JsValue::from_str("clear"))?.dyn_into::<Function>()?;
    func.call0(&storage)?.dyn_into::<Promise>()
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use js_sys::{Array, Function, Object, Reflect};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn setup_cloud_storage() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let storage = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"CloudStorage".into(), &storage);
        storage
    }

    #[wasm_bindgen_test(async)]
    async fn get_item_ok() {
        let storage = setup_cloud_storage();
        let func =
            Function::new_with_args("key", "this.called = key; return Promise.resolve('val');");
        let _ = Reflect::set(&storage, &"getItem".into(), &func);
        let value = JsFuture::from(get_item("test").unwrap()).await.unwrap();
        assert_eq!(value.as_string(), Some("val".to_string()));
        assert_eq!(
            Reflect::get(&storage, &"called".into())
                .unwrap()
                .as_string(),
            Some("test".into())
        );
    }

    #[wasm_bindgen_test]
    fn get_item_err() {
        let _ = setup_cloud_storage();
        assert!(get_item("test").is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn set_item_ok() {
        let storage = setup_cloud_storage();
        let func = Function::new_with_args(
            "key, value",
            "this.called = key + ':' + value; return Promise.resolve();"
        );
        let _ = Reflect::set(&storage, &"setItem".into(), &func);
        JsFuture::from(set_item("a", "b").unwrap()).await.unwrap();
        assert_eq!(
            Reflect::get(&storage, &"called".into())
                .unwrap()
                .as_string(),
            Some("a:b".into())
        );
    }

    #[wasm_bindgen_test]
    fn set_item_err() {
        let _ = setup_cloud_storage();
        assert!(set_item("a", "b").is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn remove_item_ok() {
        let storage = setup_cloud_storage();
        let func = Function::new_with_args("key", "this.called = key; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"removeItem".into(), &func);
        JsFuture::from(remove_item("k").unwrap()).await.unwrap();
        assert_eq!(
            Reflect::get(&storage, &"called".into())
                .unwrap()
                .as_string(),
            Some("k".into())
        );
    }

    #[wasm_bindgen_test]
    fn remove_item_err() {
        let _ = setup_cloud_storage();
        assert!(remove_item("k").is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn get_items_ok() {
        let storage = setup_cloud_storage();
        let func = Function::new_with_args(
            "keys",
            "this.called = keys; return Promise.resolve({a: '1', b: '2'});"
        );
        let _ = Reflect::set(&storage, &"getItems".into(), &func);
        let result = JsFuture::from(get_items(&["a", "b"]).unwrap())
            .await
            .unwrap();
        let obj = result.dyn_into::<Object>().unwrap();
        assert_eq!(
            Reflect::get(&obj, &"a".into()).unwrap().as_string(),
            Some("1".into())
        );
        assert_eq!(
            Reflect::get(&obj, &"b".into()).unwrap().as_string(),
            Some("2".into())
        );
        let called = Reflect::get(&storage, &"called".into()).unwrap();
        let arr = Array::from(&called);
        assert_eq!(arr.get(0).as_string(), Some("a".into()));
        assert_eq!(arr.get(1).as_string(), Some("b".into()));
    }

    #[wasm_bindgen_test]
    fn get_items_err() {
        let _ = setup_cloud_storage();
        assert!(get_items(&["a"]).is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn set_items_ok() {
        let storage = setup_cloud_storage();
        let func =
            Function::new_with_args("items", "this.called = items; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"setItems".into(), &func);
        JsFuture::from(set_items(&[("a", "1"), ("b", "2")]).unwrap())
            .await
            .unwrap();
        let called = Reflect::get(&storage, &"called".into()).unwrap();
        assert_eq!(
            Reflect::get(&called, &"a".into()).unwrap().as_string(),
            Some("1".into())
        );
        assert_eq!(
            Reflect::get(&called, &"b".into()).unwrap().as_string(),
            Some("2".into())
        );
    }

    #[wasm_bindgen_test]
    fn set_items_err() {
        let _ = setup_cloud_storage();
        assert!(set_items(&[("a", "1")]).is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn remove_items_ok() {
        let storage = setup_cloud_storage();
        let func =
            Function::new_with_args("keys", "this.called = keys; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"removeItems".into(), &func);
        JsFuture::from(remove_items(&["a", "b"]).unwrap())
            .await
            .unwrap();
        let called = Reflect::get(&storage, &"called".into()).unwrap();
        let arr = Array::from(&called);
        assert_eq!(arr.get(0).as_string(), Some("a".into()));
        assert_eq!(arr.get(1).as_string(), Some("b".into()));
    }

    #[wasm_bindgen_test]
    fn remove_items_err() {
        let _ = setup_cloud_storage();
        assert!(remove_items(&["a"]).is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn get_keys_ok() {
        let storage = setup_cloud_storage();
        let func = Function::new_no_args("return Promise.resolve(['x', 'y']);");
        let _ = Reflect::set(&storage, &"getKeys".into(), &func);
        let result = JsFuture::from(get_keys().unwrap()).await.unwrap();
        let arr = Array::from(&result);
        assert_eq!(arr.get(0).as_string(), Some("x".into()));
        assert_eq!(arr.get(1).as_string(), Some("y".into()));
    }

    #[wasm_bindgen_test]
    fn get_keys_err() {
        let _ = setup_cloud_storage();
        assert!(get_keys().is_err());
    }

    #[wasm_bindgen_test(async)]
    async fn clear_ok() {
        let storage = setup_cloud_storage();
        let func = Function::new_no_args("this.called = true; return Promise.resolve();");
        let _ = Reflect::set(&storage, &"clear".into(), &func);
        JsFuture::from(clear().unwrap()).await.unwrap();
        assert!(
            Reflect::get(&storage, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    fn clear_err() {
        let _ = setup_cloud_storage();
        assert!(clear().is_err());
    }
}
