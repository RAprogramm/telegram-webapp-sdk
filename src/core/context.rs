use once_cell::unsync::OnceCell;

use super::types::{init_data::TelegramInitData, theme_params::TelegramThemeParams};

/// Global context of the Telegram Mini App, initialized once per app session.
pub struct TelegramContext {
    pub init_data:    TelegramInitData,
    pub theme_params: TelegramThemeParams
}

thread_local! {
    /// Thread-local global TelegramContext instance.
    static CONTEXT: OnceCell<TelegramContext> = OnceCell::new();
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
