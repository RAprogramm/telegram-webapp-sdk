// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use once_cell::unsync::OnceCell;
use percent_encoding::{percent_decode, percent_decode_str};
use wasm_bindgen::JsValue;

use super::types::{
    init_data::TelegramInitData, launch_params::LaunchParams, theme_params::TelegramThemeParams
};

/// Global context of the Telegram Mini App, initialized once per app session.
#[derive(Clone)]
pub struct TelegramContext {
    pub init_data:    TelegramInitData,
    pub theme_params: TelegramThemeParams
}

thread_local! {
    /// Thread-local global TelegramContext instance.
    static CONTEXT: OnceCell<TelegramContext> = const { OnceCell::new() };
}

impl TelegramContext {
    /// Initializes the global Telegram context.
    ///
    /// # Errors
    /// Returns an error if the context was already initialized.
    pub fn init(
        init_data: TelegramInitData,
        theme_params: TelegramThemeParams
    ) -> Result<(), &'static str> {
        CONTEXT.with(|cell| {
            cell.set(TelegramContext {
                init_data,
                theme_params
            })
            .map_err(|_| "TelegramContext already initialized")
        })
    }

    /// Access the global context if it has been initialized.
    ///
    /// Accepts a closure and returns the result of applying it to the context.
    pub fn get<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&TelegramContext) -> R
    {
        CONTEXT.with(|cell| cell.get().map(f))
    }
}

/// Returns launch parameters parsed from the current window location.
///
/// The `tg_web_app_platform` entry is read from the `tgWebAppPlatform`
/// query parameter and falls back to `"web"` when it is absent.
///
/// # Errors
/// Returns a [`JsValue`] if the global window object is unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::core::context::get_launch_params;
/// let _ = get_launch_params();
/// ```
pub fn get_launch_params() -> Result<LaunchParams, JsValue> {
    let _ = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;

    Ok(LaunchParams {
        tg_web_app_platform:      get_param("tgWebAppPlatform")
            .or_else(|| Some(String::from("web"))),
        tg_web_app_version:       get_param("tgWebAppVersion"),
        tg_web_app_start_param:   get_param("tgWebAppStartParam"),
        tg_web_app_show_settings: get_param("tgWebAppShowSettings").map(|s| s == "1"),
        tg_web_app_bot_inline:    get_param("tgWebAppBotInline").map(|s| s == "1")
    })
}

fn get_param(key: &str) -> Option<String> {
    let search = web_sys::window()?.document()?.location()?.search().ok()?;

    let query = search.strip_prefix('?').unwrap_or(search.as_str());
    extract_param(query, key)
}

fn extract_param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|pair| {
        if pair.is_empty() {
            return None;
        }

        let mut parts = pair.splitn(2, '=');
        let current_key = parts.next()?;
        if current_key != key {
            return None;
        }

        let raw_value = parts.next()?;
        decode_query_value(raw_value)
    })
}

fn decode_query_value(raw_value: &str) -> Option<String> {
    if raw_value.contains('+') {
        let mut buffer = Vec::with_capacity(raw_value.len());
        for byte in raw_value.as_bytes() {
            if *byte == b'+' {
                buffer.push(b' ');
            } else {
                buffer.push(*byte);
            }
        }

        return percent_decode(buffer.as_slice())
            .decode_utf8()
            .ok()
            .map(|cow| cow.into_owned());
    }

    percent_decode_str(raw_value)
        .decode_utf8()
        .ok()
        .map(|cow| cow.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_param_returns_first_entry() {
        let query = "tgWebAppPlatform=android&tgWebAppVersion=9.2";
        let platform = extract_param(query, "tgWebAppPlatform");
        assert_eq!(platform.as_deref(), Some("android"));
    }

    #[test]
    fn decode_query_value_handles_plus_and_percent_sequences() {
        let query = "tgWebAppStartParam=hello%2Bworld+test";
        let value = extract_param(query, "tgWebAppStartParam");
        assert_eq!(value.as_deref(), Some("hello+world test"));
    }

    #[cfg(target_arch = "wasm32")]
    mod wasm {
        use wasm_bindgen::JsValue;
        use wasm_bindgen_test::wasm_bindgen_test;

        use super::super::get_launch_params;

        #[allow(dead_code)]
        #[wasm_bindgen_test]
        fn get_launch_params_returns_error_without_window() {
            let err = get_launch_params().unwrap_err();
            assert_eq!(err, JsValue::from_str("no window"));
        }

        #[wasm_bindgen_test]
        fn get_launch_params_reads_first_query_parameter() -> Result<(), JsValue> {
            let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
            let location = window.location();
            let original_search = location.search().unwrap_or_default();

            location.set_search(
                "?tgWebAppPlatform=android&tgWebAppVersion=9.2&tgWebAppStartParam=hello%2Bworld+test&tgWebAppShowSettings=1&tgWebAppBotInline=0"
            )?;

            let params = get_launch_params()?;
            assert_eq!(params.tg_web_app_platform.as_deref(), Some("android"));
            assert_eq!(params.tg_web_app_version.as_deref(), Some("9.2"));
            assert_eq!(
                params.tg_web_app_start_param.as_deref(),
                Some("hello+world test")
            );
            assert_eq!(params.tg_web_app_show_settings, Some(true));
            assert_eq!(params.tg_web_app_bot_inline, Some(false));

            location.set_search(&original_search)?;
            Ok(())
        }
    }
}
