use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Calls `Telegram.WebApp.BiometricManager.init()`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::init;
///
/// let _ = init();
/// ```
pub fn init() -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("init"))?.dyn_into::<Function>()?;
    func.call0(&biom)?;
    Ok(())
}

/// Calls `Telegram.WebApp.BiometricManager.requestAccess(auth_key, reason,
/// options)`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::request_access;
///
/// let _ = request_access("auth-key", None, None);
/// ```
pub fn request_access(
    auth_key: &str,
    reason: Option<&str>,
    options: Option<&JsValue>
) -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("requestAccess"))?.dyn_into::<Function>()?;
    let key = JsValue::from_str(auth_key);
    match (reason, options) {
        (Some(r), Some(o)) => {
            let r = JsValue::from_str(r);
            func.call3(&biom, &key, &r, o)?;
        }
        (Some(r), None) => {
            let r = JsValue::from_str(r);
            func.call2(&biom, &key, &r)?;
        }
        (None, Some(o)) => {
            func.call3(&biom, &key, &JsValue::UNDEFINED, o)?;
        }
        (None, None) => {
            func.call1(&biom, &key)?;
        }
    }
    Ok(())
}

/// Calls `Telegram.WebApp.BiometricManager.authenticate(auth_key, reason,
/// options)`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::authenticate;
///
/// let _ = authenticate("auth-key", None, None);
/// ```
pub fn authenticate(
    auth_key: &str,
    reason: Option<&str>,
    options: Option<&JsValue>
) -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("authenticate"))?.dyn_into::<Function>()?;
    let key = JsValue::from_str(auth_key);
    match (reason, options) {
        (Some(r), Some(o)) => {
            let r = JsValue::from_str(r);
            func.call3(&biom, &key, &r, o)?;
        }
        (Some(r), None) => {
            let r = JsValue::from_str(r);
            func.call2(&biom, &key, &r)?;
        }
        (None, Some(o)) => {
            func.call3(&biom, &key, &JsValue::UNDEFINED, o)?;
        }
        (None, None) => {
            func.call1(&biom, &key)?;
        }
    }
    Ok(())
}

/// Calls `Telegram.WebApp.BiometricManager.updateBiometricToken(token)`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::update_biometric_token;
///
/// let _ = update_biometric_token("token");
/// ```
pub fn update_biometric_token(token: &str) -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func =
        Reflect::get(&biom, &JsValue::from_str("updateBiometricToken"))?.dyn_into::<Function>()?;
    func.call1(&biom, &JsValue::from_str(token))?;
    Ok(())
}

/// Calls `Telegram.WebApp.BiometricManager.openSettings()`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::open_settings;
///
/// let _ = open_settings();
/// ```
pub fn open_settings() -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("openSettings"))?.dyn_into::<Function>()?;
    func.call0(&biom)?;
    Ok(())
}

/// Returns `Telegram.WebApp.BiometricManager.isInited`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a boolean.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::is_inited;
///
/// let _ = is_inited();
/// ```
pub fn is_inited() -> Result<bool, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("isInited"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isInited not a bool"))
}

/// Returns `Telegram.WebApp.BiometricManager.isBiometricAvailable`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a boolean.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::is_biometric_available;
///
/// let _ = is_biometric_available();
/// ```
pub fn is_biometric_available() -> Result<bool, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("isBiometricAvailable"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isBiometricAvailable not a bool"))
}

/// Returns `Telegram.WebApp.BiometricManager.isAccessRequested`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a boolean.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::is_access_requested;
///
/// let _ = is_access_requested();
/// ```
pub fn is_access_requested() -> Result<bool, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("isAccessRequested"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isAccessRequested not a bool"))
}

/// Returns `Telegram.WebApp.BiometricManager.isAccessGranted`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a boolean.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::is_access_granted;
///
/// let _ = is_access_granted();
/// ```
pub fn is_access_granted() -> Result<bool, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("isAccessGranted"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isAccessGranted not a bool"))
}

/// Returns `Telegram.WebApp.BiometricManager.isBiometricTokenSaved`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a boolean.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::is_biometric_token_saved;
///
/// let _ = is_biometric_token_saved();
/// ```
pub fn is_biometric_token_saved() -> Result<bool, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("isBiometricTokenSaved"))?;
    value
        .as_bool()
        .ok_or_else(|| JsValue::from_str("isBiometricTokenSaved not a bool"))
}

/// Returns `Telegram.WebApp.BiometricManager.deviceId`.
///
/// # Errors
/// Returns `Err(JsValue)` if the property is unavailable or not a string.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::device_id;
///
/// let _ = device_id();
/// ```
pub fn device_id() -> Result<String, JsValue> {
    let biom = biometric_object()?;
    let value = Reflect::get(&biom, &JsValue::from_str("deviceId"))?;
    value
        .as_string()
        .ok_or_else(|| JsValue::from_str("deviceId not a string"))
}

fn biometric_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("BiometricManager"))
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
    fn setup_biometric() -> Object {
        let win = window().expect("window should be available");
        let telegram = Object::new();
        let webapp = Object::new();
        let biom = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"BiometricManager".into(), &biom);
        biom
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn init_ok() {
        let biom = setup_biometric();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&biom, &"init".into(), &func);
        assert!(init().is_ok());
        assert!(
            Reflect::get(&biom, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn init_err() {
        let _ = setup_biometric();
        assert!(init().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_access_ok() {
        let biom = setup_biometric();
        let func = Function::new_with_args("key", "this.called = true; this.key = key;");
        let _ = Reflect::set(&biom, &"requestAccess".into(), &func);
        assert!(request_access("abc", None, None).is_ok());
        assert!(
            Reflect::get(&biom, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
        assert_eq!(
            Reflect::get(&biom, &"key".into())
                .unwrap()
                .as_string()
                .unwrap(),
            "abc"
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_access_err() {
        let _ = setup_biometric();
        assert!(request_access("abc", None, None).is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn authenticate_ok() {
        let biom = setup_biometric();
        let func = Function::new_with_args("key", "this.called = true; this.key = key;");
        let _ = Reflect::set(&biom, &"authenticate".into(), &func);
        assert!(authenticate("abc", None, None).is_ok());
        assert!(
            Reflect::get(&biom, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
        assert_eq!(
            Reflect::get(&biom, &"key".into())
                .unwrap()
                .as_string()
                .unwrap(),
            "abc"
        );
        assert_eq!(
            Reflect::get(&biom, &"reason".into())
                .unwrap()
                .as_string()
                .unwrap(),
            "why"
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn authenticate_err() {
        let _ = setup_biometric();
        assert!(authenticate("abc", None, None).is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn update_biometric_token_ok() {
        let biom = setup_biometric();
        let func = Function::new_with_args("token", "this.token = token;");
        let _ = Reflect::set(&biom, &"updateBiometricToken".into(), &func);
        assert!(update_biometric_token("abc").is_ok());
        assert_eq!(
            Reflect::get(&biom, &"token".into())
                .unwrap()
                .as_string()
                .unwrap(),
            "abc"
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn update_biometric_token_err() {
        let _ = setup_biometric();
        assert!(update_biometric_token("abc").is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_settings_ok() {
        let biom = setup_biometric();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&biom, &"openSettings".into(), &func);
        assert!(open_settings().is_ok());
        assert!(
            Reflect::get(&biom, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_settings_err() {
        let _ = setup_biometric();
        assert!(open_settings().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_inited_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"isInited".into(), &JsValue::from(true));
        assert!(is_inited().expect("is_inited"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_inited_err() {
        let _ = setup_biometric();
        assert!(is_inited().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_biometric_available_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"isBiometricAvailable".into(), &JsValue::from(true));
        assert!(is_biometric_available().expect("is_biometric_available"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_biometric_available_err() {
        let _ = setup_biometric();
        assert!(is_biometric_available().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_access_requested_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"isAccessRequested".into(), &JsValue::from(true));
        assert!(is_access_requested().expect("is_access_requested"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_access_requested_err() {
        let _ = setup_biometric();
        assert!(is_access_requested().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_access_granted_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"isAccessGranted".into(), &JsValue::from(true));
        assert!(is_access_granted().expect("is_access_granted"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_access_granted_err() {
        let _ = setup_biometric();
        assert!(is_access_granted().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_biometric_token_saved_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"isBiometricTokenSaved".into(), &JsValue::from(true));
        assert!(is_biometric_token_saved().expect("is_biometric_token_saved"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn is_biometric_token_saved_err() {
        let _ = setup_biometric();
        assert!(is_biometric_token_saved().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn device_id_ok() {
        let biom = setup_biometric();
        let _ = Reflect::set(&biom, &"deviceId".into(), &JsValue::from_str("id123"));
        assert_eq!(device_id().expect("device_id"), "id123");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn device_id_err() {
        let _ = setup_biometric();
        assert!(device_id().is_err());
    }
}
