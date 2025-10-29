// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

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

/// Typed initialization errors for better error handling and debugging.
#[derive(Debug, Clone, PartialEq)]
pub enum InitError {
    /// Browser `window` object is not available
    WindowUnavailable,
    /// `window.Telegram` is undefined
    TelegramUnavailable,
    /// `Telegram.WebApp` is undefined
    WebAppUnavailable,
    /// Failed to parse `WebApp.initData`
    InitDataParseFailed(String),
    /// Failed to parse theme parameters
    ThemeParamsParseFailed(String),
    /// Failed to initialize global context
    ContextInitFailed(String)
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowUnavailable => write!(f, "Browser window object is not available"),
            Self::TelegramUnavailable => write!(f, "window.Telegram is undefined"),
            Self::WebAppUnavailable => write!(f, "Telegram.WebApp is undefined"),
            Self::InitDataParseFailed(msg) => write!(f, "Failed to parse initData: {msg}"),
            Self::ThemeParamsParseFailed(msg) => {
                write!(f, "Failed to parse theme parameters: {msg}")
            }
            Self::ContextInitFailed(msg) => write!(f, "Failed to initialize context: {msg}")
        }
    }
}

impl std::error::Error for InitError {}

impl From<InitError> for JsValue {
    fn from(err: InitError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

/// Check if Telegram WebApp environment is available.
///
/// Returns `true` if `window.Telegram.WebApp` exists and is defined.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::core::init::is_telegram_available;
///
/// if is_telegram_available() {
///     println!("Running inside Telegram Mini App");
/// } else {
///     println!("Running in regular browser");
/// }
/// ```
pub fn is_telegram_available() -> bool {
    window()
        .and_then(|w| Reflect::get(&w, &"Telegram".into()).ok())
        .filter(|tg| !tg.is_undefined())
        .and_then(|tg| Reflect::get(&tg, &"WebApp".into()).ok())
        .filter(|webapp| !webapp.is_undefined())
        .is_some()
}

/// Attempt to initialize SDK without panicking if Telegram environment is
/// unavailable.
///
/// Returns:
/// - `Ok(true)` if SDK was successfully initialized
/// - `Ok(false)` if Telegram environment is not available (graceful
///   degradation)
/// - `Err(InitError)` for actual initialization failures
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::core::init::try_init_sdk;
///
/// match try_init_sdk() {
///     Ok(true) => println!("SDK initialized successfully"),
///     Ok(false) => println!("Not running in Telegram, using fallback"),
///     Err(e) => eprintln!("Initialization error: {}", e)
/// }
/// ```
///
/// # Errors
/// Returns typed `InitError` for parsing failures or context initialization
/// issues.
pub fn try_init_sdk() -> Result<bool, InitError> {
    if !is_telegram_available() {
        return Ok(false);
    }
    init_sdk_typed().map(|_| true)
}

/// Internal typed version of init_sdk for use by try_init_sdk.
fn init_sdk_typed() -> Result<(), InitError> {
    let win = window().ok_or(InitError::WindowUnavailable)?;
    let telegram =
        Reflect::get(&win, &"Telegram".into()).map_err(|_| InitError::TelegramUnavailable)?;

    if telegram.is_undefined() {
        return Err(InitError::TelegramUnavailable);
    }

    let webapp =
        Reflect::get(&telegram, &"WebApp".into()).map_err(|_| InitError::WebAppUnavailable)?;

    if webapp.is_undefined() {
        return Err(InitError::WebAppUnavailable);
    }

    // === 1. Parse initData string ===
    let init_data_str = Reflect::get(&webapp, &"initData".into())
        .ok()
        .and_then(|v| v.as_string())
        .ok_or_else(|| InitError::InitDataParseFailed("initData is not a string".to_string()))?;

    let raw: TelegramInitDataInternal = serde_urlencoded::from_str(&init_data_str)
        .map_err(|e| InitError::InitDataParseFailed(e.to_string()))?;

    // === 2. Parse embedded JSON fields ===
    let user: Option<TelegramUser> = raw
        .user
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| InitError::InitDataParseFailed(format!("Failed to parse user: {e}")))?;

    let receiver: Option<TelegramUser> = raw
        .receiver
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| InitError::InitDataParseFailed(format!("Failed to parse receiver: {e}")))?;

    let chat: Option<TelegramChat> = raw
        .chat
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| InitError::InitDataParseFailed(format!("Failed to parse chat: {e}")))?;

    // === 3. Construct final typed initData ===
    let init_data = TelegramInitData {
        query_id: raw.query_id,
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
    let theme_val = Reflect::get(&webapp, &"themeParams".into())
        .map_err(|e| InitError::ThemeParamsParseFailed(format!("{e:?}")))?;
    let theme_params: TelegramThemeParams =
        from_value(theme_val).map_err(|e| InitError::ThemeParamsParseFailed(format!("{e:?}")))?;

    // === 5. Init global context ===
    TelegramContext::init(init_data, theme_params, init_data_str)
        .map_err(|e| InitError::ContextInitFailed(format!("{e:?}")))?;

    Ok(())
}

/// Initializes Telegram WebApp SDK by extracting and validating context.
///
/// - Parses `initData` (urlencoded) with embedded JSON.
/// - Parses `themeParams` (object).
/// - Initializes global context.
///
/// # Errors
///
/// Returns `Err(JsValue)` in the following cases:
///
/// - `WindowUnavailable`: No browser `window` object found
/// - `TelegramUnavailable`: `window.Telegram` is undefined
/// - `WebAppUnavailable`: `Telegram.WebApp` is undefined
/// - `InitDataParseFailed`: Failed to parse `WebApp.initData`
/// - `ThemeParamsParseFailed`: Failed to parse theme parameters
/// - `ContextInitFailed`: Failed to initialize global context
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::core::init::init_sdk;
///
/// match init_sdk() {
///     Ok(_) => println!("SDK initialized successfully"),
///     Err(e) => eprintln!("Initialization failed: {:?}", e)
/// }
/// ```
///
/// For better error handling, consider using [`try_init_sdk`] which returns
/// typed [`InitError`].
pub fn init_sdk() -> Result<(), JsValue> {
    init_sdk_typed().map_err(Into::into)
}
