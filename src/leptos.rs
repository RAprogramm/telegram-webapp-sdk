// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

/// [`back_button::BackButton`] component driving `WebApp.BackButton`.
pub mod back_button;
/// [`bottom_button::BottomButton`] component driving the main/secondary button.
pub mod bottom_button;
/// [`safe_area::use_safe_area`] hook exposing safe-area insets reactively.
pub mod safe_area;
/// [`settings_button::SettingsButton`] component driving
/// `WebApp.SettingsButton`.
pub mod settings_button;
/// [`theme::use_theme`] hook exposing Telegram theme parameters reactively.
pub mod theme;
/// [`viewport::use_viewport`] hook exposing viewport size and state reactively.
pub mod viewport;

pub use back_button::BackButton;
pub use bottom_button::BottomButton;
use leptos::prelude::provide_context;
pub use safe_area::{SafeAreaState, use_safe_area};
pub use settings_button::SettingsButton;
pub use theme::{ThemeState, use_theme};
pub use viewport::{ViewportState, use_viewport};
use wasm_bindgen::JsValue;

use crate::core::{context::TelegramContext, safe_context::get_context};

/// Provides the [`TelegramContext`] to the Leptos reactive system.
///
/// # Errors
///
/// Returns an error if the global context has not been initialized with
/// [`TelegramContext::init`].
///
/// # Examples
///
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::{core::context::TelegramContext, leptos::provide_telegram_context};
///
/// #[component]
/// fn App() -> impl IntoView {
///     provide_telegram_context().expect("context");
///     let ctx = use_context::<TelegramContext>().expect("context");
///     view! { <span>{ ctx.init_data.auth_date }</span> }
/// }
/// ```
pub fn provide_telegram_context() -> Result<(), JsValue> {
    let ctx: TelegramContext = get_context(|c| c.clone())?;
    provide_context(ctx);
    Ok(())
}
