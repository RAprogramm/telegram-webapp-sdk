use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Calls `Telegram.WebApp.requestContact()`.
///
/// Requires the user's explicit permission to share their contact information.
///
/// # Errors
/// Returns `Err(JsValue)` if `Telegram.WebApp` or the method is unavailable, or
/// if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::user::request_contact;
///
/// let _ = request_contact();
/// ```
pub fn request_contact() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func =
        Reflect::get(&webapp, &JsValue::from_str("requestContact"))?.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Calls `Telegram.WebApp.requestPhoneNumber()`.
///
/// Requires the user's explicit permission to share their phone number.
///
/// # Errors
/// Returns `Err(JsValue)` if `Telegram.WebApp` or the method is unavailable, or
/// if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::user::request_phone_number;
///
/// let _ = request_phone_number();
/// ```
pub fn request_phone_number() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func =
        Reflect::get(&webapp, &JsValue::from_str("requestPhoneNumber"))?.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

/// Calls `Telegram.WebApp.openContact()`.
///
/// Requires the user's permission to open the contact interface in Telegram.
///
/// # Errors
/// Returns `Err(JsValue)` if `Telegram.WebApp` or the method is unavailable, or
/// if the call fails.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::user::open_contact;
///
/// let _ = open_contact();
/// ```
pub fn open_contact() -> Result<(), JsValue> {
    let webapp = webapp_object()?;
    let func = Reflect::get(&webapp, &JsValue::from_str("openContact"))?.dyn_into::<Function>()?;
    func.call0(&webapp)?;
    Ok(())
}

fn webapp_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &JsValue::from_str("Telegram"))?;
    Reflect::get(&tg, &JsValue::from_str("WebApp"))
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_contact_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"requestContact".into(), &func);
        assert!(request_contact().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_contact_err() {
        let _ = setup_webapp();
        assert!(request_contact().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_phone_number_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"requestPhoneNumber".into(), &func);
        assert!(request_phone_number().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_phone_number_err() {
        let _ = setup_webapp();
        assert!(request_phone_number().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_contact_ok() {
        let webapp = setup_webapp();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&webapp, &"openContact".into(), &func);
        assert!(open_contact().is_ok());
        assert!(
            Reflect::get(&webapp, &"called".into())
                .unwrap()
                .as_bool()
                .unwrap()
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_contact_err() {
        let _ = setup_webapp();
        assert!(open_contact().is_err());
    }
}
