//! Telegram WebApp SDK macros.
//!
//! This module provides declarative macros for building Telegram WebApp
//! applications. They allow you to:
//!
//! * Register routable pages using [`telegram_page!`].
//! * Define the application entry point with Telegram SDK initialization using
//!   [`telegram_app!`].
//! * Build and start a router that automatically collects all registered pages
//!   using [`telegram_router!`].
//!
//! ## Requirements
//!
//! 1. A `Page` type and a global `inventory` collection in your crate, for
//!    example:
//!
//! ```no_run
//! pub mod pages {
//!     /// Handler type for a page: a simple `fn()`.
//!     pub type Handler = fn();
//!
//!     /// Routable page descriptor.
//!     #[derive(Copy, Clone)]
//!     pub struct Page {
//!         pub path:    &'static str,
//!         pub handler: Handler
//!     }
//!
//!     // Collect all `Page` items via `inventory`.
//!     inventory::collect!(Page);
//!
//!     /// Iterate over all collected pages.
//!     pub fn iter() -> impl Iterator<Item = &'static Page> {
//!         inventory::iter::<Page>
//!     }
//! }
//! ```
//!
//! 2. A `Router` type must be available in scope when using
//!    [`telegram_router!`], with the following API:
//!
//! ```ignore
//! impl Router {
//!     fn new() -> Self;
//!     fn register(self, path: &str, handler: fn()) -> Self;
//!     fn start(self);
//! }
//! ```
//!
//! 3. For [`telegram_app!`], the following items must exist in your crate:
//!
//! * `utils::check_env::is_telegram_env() -> bool`
//! * `mock::config::MockTelegramConfig::from_file(path) -> Result<_, _>`
//! * `mock::init::mock_telegram_webapp(cfg) -> Result<_, _>`
//! * `core::init::init_sdk() -> Result<(), wasm_bindgen::JsValue>`
//!
//! 4. Add the following dependencies to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! inventory = "0.3"
//! wasm-bindgen = "0.2"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use wasm_bindgen::prelude::JsValue;
//!
//! // Register a page.
//! telegram_webapp_sdk::telegram_page!(
//!     "/",
//!     /// Home page handler.
//!     pub fn index() {
//!         // render something
//!     }
//! );
//!
//! // Application entry point.
//! telegram_webapp_sdk::telegram_app!(
//!     /// Application main entry.
//!     pub fn main() -> Result<(), JsValue> {
//!         telegram_webapp_sdk::telegram_router!();
//!         Ok(())
//!     }
//! );
//! ```

#![allow(clippy::module_name_repetitions)]

/// Registers a routable page.
///
/// Expands into:
/// * A function definition with the provided visibility, name, and body.
/// * An `inventory::submit!` registration wrapped in a hidden module, so the
///   compiler treats it as a valid item in any context.
///
/// # Handler signature
///
/// The handler must be a plain function `fn()` with no arguments. If you need
/// state or context, encapsulate it externally instead of passing arguments.
///
/// # Example
///
/// ```no_run
/// use telegram_webapp_sdk::telegram_page;
///
/// telegram_page!(
///     "/about",
///     /// About page.
///     pub fn about() {
///         // render about page
///     }
/// );
/// ```
#[macro_export]
macro_rules! telegram_page {
    ($path:literal, $(#[$meta:meta])* $vis:vis fn $name:ident $($rest:tt)*) => {
        $(#[$meta])*
        $vis fn $name $($rest)*

        #[doc(hidden)]
        mod __telegram_page_register {
            // Keep handler reachable while hiding helper names.
            use super::$name as __handler;
            #[allow(non_upper_case_globals)]
            const _: () = {
                $crate::inventory::submit! {
                    $crate::pages::Page { path: $path, handler: __handler }
                }
            };
        }
    };
}

/// Defines the WASM application entry point with Telegram SDK initialization.
///
/// The generated function is annotated with `#[wasm_bindgen(start)]`.
/// It performs:
///
/// * Environment detection with `utils::check_env::is_telegram_env()`.
/// * Debug-only mock initialization if outside Telegram.
/// * SDK initialization via `core::init::init_sdk()?`.
///
/// After these steps, the provided function body is executed.
///
/// # Return type
///
/// The function may return either `()` or `Result<(), wasm_bindgen::JsValue>`.
///
/// # Example
///
/// ```no_run
/// use telegram_webapp_sdk::telegram_app;
/// use wasm_bindgen::JsValue;
///
/// telegram_app!(
///     /// Application entry point.
///     pub fn main() -> Result<(), JsValue> {
///         telegram_webapp_sdk::telegram_router!();
///         Ok(())
///     }
/// );
/// ```
#[macro_export]
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

/// Builds and starts a router from all registered pages.
///
/// This macro expects a `Router` type in scope with methods:
///
/// * `fn new() -> Self`
/// * `fn register(self, path: &str, handler: fn()) -> Self`
/// * `fn start(self)`
///
/// # Example
///
/// ```no_run
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
/// telegram_page!("/", pub fn index() {});
///
/// telegram_router!();
/// ```
#[macro_export]
macro_rules! telegram_router {
    () => {{
        let mut router = Router::new();
        for page in $crate::pages::iter() {
            router = router.register(page.path, page.handler);
        }
        router.start();
    }};
}
