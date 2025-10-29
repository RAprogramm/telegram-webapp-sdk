// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use yew::prelude::{hook, use_effect, use_state};

use crate::core::{context::TelegramContext, safe_context::get_context};

pub mod bottom_button;
pub use bottom_button::BottomButton;

type ClosureCell = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

/// Yew hook that reactively exposes the global [`TelegramContext`].
///
/// This hook checks for context availability at mount time and reactively
/// updates when the context becomes available. It uses `requestAnimationFrame`
/// for efficient polling until the context is initialized.
///
/// # Errors
///
/// Returns an error if the context has not been initialized with
/// [`TelegramContext::init`]. The error state is reactive and will update
/// to `Ok` once initialization completes.
///
/// # Examples
///
/// ```no_run
/// use telegram_webapp_sdk::yew::use_telegram_context;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let ctx_result = use_telegram_context();
///
///     match ctx_result.as_ref() {
///         Ok(ctx) => html! { <span>{ ctx.init_data.auth_date }</span> },
///         Err(_) => html! { <div>{"Loading Telegram context..."}</div> }
///     }
/// }
/// ```
#[hook]
pub fn use_telegram_context() -> Result<TelegramContext, JsValue> {
    let context_state = use_state(|| get_context(|c| c.clone()));

    {
        let context_state = context_state.clone();
        use_effect(move || {
            let handle: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
            let closure: ClosureCell = Rc::new(RefCell::new(None));

            if context_state.is_err()
                && let Some(win) = web_sys::window()
            {
                let handle_clone = handle.clone();
                let closure_clone = closure.clone();
                let ctx_state = context_state.clone();

                let check_fn = Closure::wrap(Box::new(move || {
                    if let Ok(ctx) = get_context(|c| c.clone()) {
                        ctx_state.set(Ok(ctx));
                        if let Some(id) = handle_clone.borrow_mut().take()
                            && let Some(w) = web_sys::window()
                        {
                            let _ = w.cancel_animation_frame(id);
                        }
                        closure_clone.borrow_mut().take();
                    } else if let Some(w) = web_sys::window()
                        && let Some(cb) = closure_clone.borrow().as_ref()
                        && let Ok(id) = w.request_animation_frame(cb.as_ref().unchecked_ref())
                    {
                        *handle_clone.borrow_mut() = Some(id);
                    }
                }) as Box<dyn FnMut()>);

                if let Ok(id) = win.request_animation_frame(check_fn.as_ref().unchecked_ref()) {
                    *handle.borrow_mut() = Some(id);
                }

                *closure.borrow_mut() = Some(check_fn);
            }

            let cleanup_handle = handle;
            let cleanup_closure = closure;
            move || {
                if let Some(id) = cleanup_handle.borrow_mut().take()
                    && let Some(w) = web_sys::window()
                {
                    let _ = w.cancel_animation_frame(id);
                }
                cleanup_closure.borrow_mut().take();
            }
        });
    }

    (*context_state).clone()
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    mod wasm {
        use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
        use yew::prelude::*;

        use super::super::use_telegram_context;
        use crate::core::{
            context::TelegramContext,
            types::{
                init_data::TelegramInitData, theme_params::TelegramThemeParams, user::TelegramUser
            }
        };

        wasm_bindgen_test_configure!(run_in_browser);

        #[function_component(TestComponent)]
        fn test_component() -> Html {
            let ctx_result = use_telegram_context();

            match ctx_result.as_ref() {
                Ok(ctx) => html! {
                    <div id="success">{ format!("auth_date: {}", ctx.init_data.auth_date) }</div>
                },
                Err(e) => html! {
                    <div id="error">{ format!("Error: {:?}", e) }</div>
                }
            }
        }

        #[wasm_bindgen_test]
        fn hook_renders_component_with_context_result() {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(container) = document.create_element("div") {
                        yew::Renderer::<TestComponent>::with_root(container).render();
                    }
                }
            }
        }

        #[wasm_bindgen_test]
        fn hook_works_with_initialized_context() {
            let init_data = TelegramInitData {
                query_id:       Some(String::from("test_query_2")),
                user:           Some(TelegramUser {
                    id: 987654321,
                    is_bot: Some(false),
                    first_name: String::from("Test2"),
                    last_name: Some(String::from("User2")),
                    username: Some(String::from("testuser2")),
                    language_code: Some(String::from("en")),
                    is_premium: Some(false),
                    added_to_attachment_menu: Some(false),
                    allows_write_to_pm: Some(true),
                    photo_url: None
                }),
                receiver:       None,
                chat:           None,
                chat_type:      None,
                chat_instance:  None,
                start_param:    None,
                can_send_after: None,
                auth_date:      9876543210,
                hash:           String::from("test_hash_2"),
                signature:      None
            };

            let theme_params = TelegramThemeParams {
                bg_color:                  Some(String::from("#000000")),
                text_color:                Some(String::from("#ffffff")),
                hint_color:                Some(String::from("#666666")),
                link_color:                Some(String::from("#00aaff")),
                button_color:              Some(String::from("#00aaff")),
                button_text_color:         Some(String::from("#000000")),
                secondary_bg_color:        Some(String::from("#1a1a1a")),
                header_bg_color:           None,
                bottom_bar_bg_color:       None,
                accent_text_color:         None,
                section_bg_color:          None,
                section_header_text_color: None,
                section_separator_color:   None,
                subtitle_text_color:       None,
                destructive_text_color:    None
            };

            let raw_init_data = String::from(
                "query_id=test_query_2&user=%7B%22id%22%3A987654321%7D&auth_date=9876543210&hash=test_hash_2"
            );

            let _ = TelegramContext::init(init_data, theme_params, raw_init_data);

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(container) = document.create_element("div") {
                        yew::Renderer::<TestComponent>::with_root(container).render();
                    }
                }
            }
        }
    }
}
