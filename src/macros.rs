#[macro_export]
/// Registers a routable page.
///
/// The macro creates the provided function and submits it to the global page
/// inventory.
///
/// # Examples
///
/// ```ignore
/// use telegram_webapp_sdk::telegram_page;
///
/// telegram_page!("/", fn index() {});
/// ```
macro_rules! telegram_page {
    ($path:literal, $(#[$meta:meta])* $vis:vis fn $name:ident $($rest:tt)*) => {
        $(#[$meta])*
        $vis fn $name $($rest)*

        $crate::inventory::submit! {
            $crate::pages::Page { path: $path, handler: $name }
        }
    };
}

#[macro_export]
/// Defines the application entry point with Telegram initialization.
///
/// Adds environment checks and SDK initialization before executing the provided
/// body.
///
/// # Examples
///
/// ```ignore
/// use telegram_webapp_sdk::{telegram_app, telegram_router};
/// use wasm_bindgen::JsValue;
///
/// telegram_app!(
///     fn main() -> Result<(), JsValue> {
///         telegram_router!();
///         Ok(())
///     }
/// );
/// ```
macro_rules! telegram_app {
    ($(#[$meta:meta])* $vis:vis fn $name:ident($($arg:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$meta])*
        #[::wasm_bindgen::prelude::wasm_bindgen(start)]
        $vis fn $name($($arg)*) $(-> $ret)? {
            if !$crate::utils::check_env::is_telegram_env() {
                #[cfg(debug_assertions)]
                if let Ok(cfg) = $crate::mock::config::MockTelegramConfig::from_file("telegram-webapp.toml") {
                    let _ = $crate::mock::init::mock_telegram_webapp(cfg);
                }
            }
            $crate::core::init::init_sdk()?;
            $body
        }
    };
}

#[macro_export]
/// Builds and starts a router using all registered pages.
///
/// A `Router` type with `new`, `register`, and `start` methods must be in
/// scope.
///
/// # Examples
///
/// ```ignore
/// use telegram_webapp_sdk::{telegram_page, telegram_router};
///
/// struct Router;
/// impl Router {
///     fn new() -> Self {
///         Router
///     }
///     fn register(self, _path: &str, _handler: fn()) -> Self {
///         self
///     }
///     fn start(self) {}
/// }
///
/// telegram_page!("/", fn index() {});
/// telegram_router!();
/// ```
macro_rules! telegram_router {
    () => {{
        let mut router = Router::new();
        for page in $crate::pages::iter() {
            router = router.register(page.path, page.handler);
        }
        router.start();
    }};
}
