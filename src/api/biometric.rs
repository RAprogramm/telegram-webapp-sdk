use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
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

/// Calls `Telegram.WebApp.BiometricManager.requestAccess()`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::request_access;
///
/// let _ = request_access();
/// ```
pub fn request_access() -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("requestAccess"))?.dyn_into::<Function>()?;
    func.call0(&biom)?;
    Ok(())
}

/// Calls `Telegram.WebApp.BiometricManager.authenticate()`.
///
/// # Errors
/// Returns `Err(JsValue)` if `BiometricManager` or the method is unavailable,
/// or if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::biometric::authenticate;
///
/// let _ = authenticate();
/// ```
pub fn authenticate() -> Result<(), JsValue> {
    let biom = biometric_object()?;
    let func = Reflect::get(&biom, &JsValue::from_str("authenticate"))?.dyn_into::<Function>()?;
    func.call0(&biom)?;
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

fn biometric_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&tg, &JsValue::from_str("WebApp"))?;
    Reflect::get(&webapp, &JsValue::from_str("BiometricManager"))
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
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
        assert!(Reflect::get(&biom, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap());
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
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&biom, &"requestAccess".into(), &func);
        assert!(request_access().is_ok());
        assert!(Reflect::get(&biom, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_access_err() {
        let _ = setup_biometric();
        assert!(request_access().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn authenticate_ok() {
        let biom = setup_biometric();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&biom, &"authenticate".into(), &func);
        assert!(authenticate().is_ok());
        assert!(Reflect::get(&biom, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn authenticate_err() {
        let _ = setup_biometric();
        assert!(authenticate().is_err());
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
        assert!(Reflect::get(&biom, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_settings_err() {
        let _ = setup_biometric();
        assert!(open_settings().is_err());
    }
}
