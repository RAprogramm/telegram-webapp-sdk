use js_sys::Reflect;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use web_sys::window;

use crate::core::{
    context::TelegramContext,
    types::{
        chat::TelegramChat, init_data::TelegramInitData,
        init_data_internal::TelegramInitDataInternal, theme_params::TelegramThemeParams,
        user::TelegramUser
    }
};

/// Initializes Telegram WebApp SDK by extracting and validating context.
///
/// - Parses `initData` (urlencoded) with embedded JSON.
/// - Parses `themeParams` (object).
/// - Initializes global context.
///
/// # Errors
/// Returns `Err(JsValue)` on failure to access JS globals, parse, or init
/// context.
pub fn init_sdk() -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("window is not available"))?;
    let telegram = Reflect::get(&win, &"Telegram".into())?;
    let webapp = Reflect::get(&telegram, &"WebApp".into())?;

    // === 1. Parse initData string ===
    let init_data_str = Reflect::get(&webapp, &"initData".into())?
        .as_string()
        .ok_or_else(|| JsValue::from_str("Telegram.WebApp.initData is not a string"))?;

    let raw: TelegramInitDataInternal = serde_urlencoded::from_str(&init_data_str)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse initData: {e}")))?;

    // === 2. Parse embedded JSON fields ===
    let user: Option<TelegramUser> = raw
        .user
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse user: {e}")))?;

    let receiver: Option<TelegramUser> = raw
        .receiver
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse receiver: {e}")))?;

    let chat: Option<TelegramChat> = raw
        .chat
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| JsValue::from_str(&format!("Failed to parse chat: {e}")))?;

    // === 3. Construct final typed initData ===
    let init_data = TelegramInitData {
        query_id: None, // not available in urlencoded format
        user,
        receiver,
        chat,
        chat_type: raw.chat_type,
        chat_instance: raw.chat_instance,
        start_param: raw.start_param,
        can_send_after: raw.can_send_after,
        auth_date: raw.auth_date,
        hash: raw.hash,
        signature: raw.signature
    };

    // === 4. Parse themeParams ===
    let theme_val = Reflect::get(&webapp, &"themeParams".into())?;
    let theme_params: TelegramThemeParams = from_value(theme_val)?;

    // theme_params.clone().apply_to_root();

    // === 5. Init global context ===
    TelegramContext::init(init_data, theme_params)?;

    Ok(())
}
