// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

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
}
