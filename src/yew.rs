use wasm_bindgen::JsValue;
use yew::prelude::{hook, use_memo};

use crate::core::{context::TelegramContext, safe_context::get_context};

/// Yew hook that exposes the global [`TelegramContext`].
///
/// # Errors
///
/// Returns an error if the context has not been initialized with
/// [`TelegramContext::init`].
///
/// # Examples
///
/// ```no_run
/// use telegram_webapp_sdk::yew::use_telegram_context;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let ctx = use_telegram_context().expect("context");
///     html! { <span>{ ctx.init_data.auth_date }</span> }
/// }
/// ```
#[hook]
pub fn use_telegram_context() -> Result<TelegramContext, JsValue> {
    let ctx = use_memo((), |_| get_context(|c| c.clone()));
    (*ctx).clone()
}
