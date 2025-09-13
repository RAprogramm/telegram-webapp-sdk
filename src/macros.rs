//! Telegram WebApp SDK macros.
//!
//! This module provides declarative macros for building Telegram WebApp
//! applications. They let you:
//!
//! * Register routable pages using [`telegram_page!`]
//! * Define the WASM application entry point with Telegram SDK initialization
//!   using [`telegram_app!`]
//! * Build and start a router that collects all registered pages via
//!   `inventory` using [`telegram_router!`]
//!
//! ## Requirements
//!
//! 1. A `Page` type and a global `inventory` collection in your crate, for
//!    example:
//!
//! ```ignore
//! pub mod pages {
//!     /// Handler type for a page: a plain `fn()`.
//!     pub type Handler = fn();
//!
//!     /// Routable page descriptor.
//!     #[derive(Copy, Clone)]
//!     pub struct Page {
//!         pub path: &'static str,
//!         pub handler: Handler;
//!     }
//!
//!     // Collect all `Page` items via `inventory`.
//!     inventory::collect!(Page);
//!
//!     /// Iterate over all collected pages as a real `Iterator`.
//!     pub fn iter() -> impl Iterator<Item = &'static Page> {
//!         inventory::iter::<Page>.into_iter()
//!     }
//! }
//! ```
//!
//! 2. [`telegram_router!`] uses [`crate::router::Router`] by default. To supply
//!    a custom router type, ensure it exposes:
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
//! 4. `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! inventory = "0.3"
//! wasm-bindgen = "0.2"
//! ```
//!
//! ## Quick example
//!
//! ```ignore
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

/// Register a routable page.
///
/// Expands into:
/// * A function definition with the provided visibility, name, and body
/// * A single registration item that submits a [`crate::pages::Page`] to
///   `inventory`, wrapped in a hidden module to remain a valid item in any
///   context
///
/// ### Handler signature
///
/// The handler must be a plain function `fn()` with no arguments. If you need
/// state or context, encapsulate it externally (e.g. closures, singletons, DI),
/// not as handler parameters.
///
/// ### Example
///
/// ```ignore
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

/// Define the WASM application entry point with Telegram SDK initialization.
///
/// The generated function is annotated with `#[wasm_bindgen(start)]`.
/// It performs:
///
/// * Environment detection via `utils::check_env::is_telegram_env()`
/// * Debug-only mock initialization when not in Telegram
/// * SDK initialization via `core::init::init_sdk()?`
///
/// After these steps, the provided function body is executed.
///
/// ### Return type
///
/// The function may return either `()` or `Result<(), wasm_bindgen::JsValue>`.
///
/// ### Example
///
/// ```ignore
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
        #[wasm_bindgen::prelude::wasm_bindgen(start)]
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

/// Build and start a router from all registered pages.
///
/// By default it uses [`crate::router::Router`]. A custom router type can be
/// supplied as the first argument. The router type must expose:
///
/// * `fn new() -> Self`
/// * `fn register(self, path: &str, handler: fn()) -> Self`
/// * `fn start(self)`
///
/// ### Examples
///
/// Using the default router:
///
/// ```ignore
/// use telegram_webapp_sdk::{telegram_page, telegram_router};
///
/// telegram_page!("/", pub fn index() {});
/// telegram_router!();
/// ```
///
/// Providing a custom router type:
///
/// ```ignore
/// use telegram_webapp_sdk::telegram_router;
///
/// struct CustomRouter;
/// impl CustomRouter {
///     fn new() -> Self { CustomRouter }
///     fn register(self, _path: &str, _handler: fn()) -> Self { self }
///     fn start(self) {}
/// }
///
/// telegram_router!(CustomRouter);
/// ```
#[macro_export]
macro_rules! telegram_router {
    () => {
        $crate::telegram_router!($crate::router::Router);
    };
    ($router:ty) => {{
        let mut router = <$router>::new();
        for page in $crate::pages::iter() {
            router = router.register(page.path, page.handler);
        }
        router.start();
    }};
}

/// Create a `<button>` element.
///
/// Generates a [`web_sys::HtmlElement`] with the provided text, optional CSS
/// class and arbitrary attributes. The macro evaluates to
/// `Result<web_sys::HtmlElement, wasm_bindgen::JsValue>` so it can be used with
/// the `?` operator inside functions returning `Result`.
///
/// # Examples
///
/// ```ignore
/// use telegram_webapp_sdk::telegram_button;
/// use wasm_bindgen::JsValue;
///
/// # fn example() -> Result<(), JsValue> {
/// let document = web_sys::window()
///     .and_then(|w| w.document())
///     .ok_or_else(|| JsValue::from_str("no document"))?;
/// let button = telegram_button!(document, "Click", class = "primary", "type" = "button")?;
/// assert_eq!(button.tag_name(), "BUTTON");
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! telegram_button {
    ($doc:expr, $text:expr $(, class = $class:expr)? $(, $attr:literal = $value:expr)* $(,)?) => {{
        || -> Result<web_sys::HtmlElement, wasm_bindgen::JsValue> {
            use wasm_bindgen::JsCast;
            let element = $doc.create_element("button")?;
            element.set_inner_html($text);
            $(element.set_class_name($class);)?
            $(
                element.set_attribute($attr, $value)?;
            )*
            element
                .dyn_into::<web_sys::HtmlElement>()
                .map_err(wasm_bindgen::JsValue::from)
        }()
    }};
}

/// Create an `<img>` element.
///
/// Generates a [`web_sys::HtmlImageElement`] with the provided `src`, optional
/// CSS class, `alt` text and additional attributes. Like
/// [`telegram_button!`], this macro yields a `Result` for ergonomic error
/// propagation.
///
/// # Examples
///
/// ```ignore
/// use telegram_webapp_sdk::telegram_image;
/// use wasm_bindgen::JsValue;
///
/// # fn example() -> Result<(), JsValue> {
/// let document = web_sys::window()
///     .and_then(|w| w.document())
///     .ok_or_else(|| JsValue::from_str("no document"))?;
/// let image = telegram_image!(document, "/logo.png", class = "logo", alt = "Logo")?;
/// assert_eq!(image.tag_name(), "IMG");
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! telegram_image {
    ($doc:expr, $src:expr $(, class = $class:expr)? $(, alt = $alt:expr)? $(, $attr:literal = $value:expr)* $(,)?) => {{
        || -> Result<web_sys::HtmlImageElement, wasm_bindgen::JsValue> {
            use wasm_bindgen::JsCast;
            let element = $doc.create_element("img")?;
            element.set_attribute("src", $src)?;
            $(element.set_class_name($class);)?
            $(element.set_attribute("alt", $alt)?;)?
            $(
                element.set_attribute($attr, $value)?;
            )*
            element
                .dyn_into::<web_sys::HtmlImageElement>()
                .map_err(wasm_bindgen::JsValue::from)
        }()
    }};
}
