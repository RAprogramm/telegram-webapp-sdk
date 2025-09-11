use once_cell::unsync::OnceCell;
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
/// # Errors
/// Returns a [`JsValue`] if the global window object is unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::core::context::get_launch_params;
/// let _ = get_launch_params();
/// ```
pub fn get_launch_params() -> Result<LaunchParams, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let location = window.location();

    Ok(LaunchParams {
        tg_web_app_platform:      location.origin().ok().or_else(|| Some("web".into())),
        tg_web_app_version:       get_param("tgWebAppVersion"),
        tg_web_app_start_param:   get_param("tgWebAppStartParam"),
        tg_web_app_show_settings: get_param("tgWebAppShowSettings").map(|s| s == "1"),
        tg_web_app_bot_inline:    get_param("tgWebAppBotInline").map(|s| s == "1")
    })
}

fn get_param(key: &str) -> Option<String> {
    web_sys::window()?
        .document()?
        .location()?
        .search()
        .ok()?
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.split('=');
            let k = parts.next()?;
            let v = parts.next()?;
            if k == key { Some(v.to_string()) } else { None }
        })
}

#[cfg(test)]
mod tests {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn get_launch_params_returns_error_without_window() {
        let err = get_launch_params().unwrap_err();
        assert_eq!(err, JsValue::from_str("no window"));
    }
}
