use wasm_bindgen::JsValue;

use crate::core::context::TelegramContext;

pub fn get_context<T>(f: impl FnOnce(&TelegramContext) -> T) -> Result<T, JsValue> {
    TelegramContext::get(f).ok_or_else(|| JsValue::from_str("TelegramContext is not initialized"))
}
