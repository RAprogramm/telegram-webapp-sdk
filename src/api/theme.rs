use js_sys::Reflect;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use web_sys::window;

use crate::core::types::theme_params::TelegramThemeParams;

/// Returns the current themeParams from `Telegram.WebApp.themeParams`.
///
/// # Errors
/// Returns `Err(JsValue)` if the object is missing or cannot be parsed.
pub fn get_theme_params() -> Result<TelegramThemeParams, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let telegram = Reflect::get(&window, &JsValue::from_str("Telegram"))?;
    let webapp = Reflect::get(&telegram, &JsValue::from_str("WebApp"))?;
    let theme_params = Reflect::get(&webapp, &JsValue::from_str("themeParams"))?;
    from_value(theme_params)
        .map_err(|e| JsValue::from_str(&format!("themeParams parse error: {e}")))
}

#[cfg(test)]
mod tests {
    use js_sys::{Object, Reflect};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().expect("window should be available");
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn parses_valid_theme() {
        let webapp = setup_webapp();
        let theme = Object::new();
        let _ = Reflect::set(&theme, &"bg_color".into(), &JsValue::from_str("#ffffff"));
        let _ = Reflect::set(&theme, &"text_color".into(), &JsValue::from_str("#000000"));
        let _ = Reflect::set(&webapp, &"themeParams".into(), &theme);

        let params = get_theme_params().expect("theme params");
        assert_eq!(params.bg_color.as_deref(), Some("#ffffff"));
        assert_eq!(params.text_color.as_deref(), Some("#000000"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn fails_on_invalid_data() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"themeParams".into(), &JsValue::from_f64(5.0));
        assert!(get_theme_params().is_err());
    }
}
