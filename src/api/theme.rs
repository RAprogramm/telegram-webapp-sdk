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
