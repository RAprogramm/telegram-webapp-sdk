use leptos::prelude::provide_context;
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
