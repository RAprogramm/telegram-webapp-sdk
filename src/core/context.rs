use once_cell::unsync::OnceCell;

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

pub fn get_launch_params() -> LaunchParams {
    let window = web_sys::window().expect("no window");
    let location = window.location();

    LaunchParams {
        tg_web_app_platform:      location.origin().ok().or_else(|| Some("web".into())),
        tg_web_app_version:       get_param("tgWebAppVersion"),
        tg_web_app_start_param:   get_param("tgWebAppStartParam"),
        tg_web_app_show_settings: get_param("tgWebAppShowSettings").map(|s| s == "1"),
        tg_web_app_bot_inline:    get_param("tgWebAppBotInline").map(|s| s == "1")
    }
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
