// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::cell::RefCell;

use yew::prelude::{Callback, Html, Properties, function_component, html, use_effect_with};

use crate::{
    logger,
    webapp::{EventHandle, TelegramWebApp}
};

thread_local! {
    static BACK_BUTTON_HANDLE: RefCell<Option<EventHandle<dyn FnMut()>>> =
        const { RefCell::new(None) };
}

/// Props for [`BackButton`].
#[derive(Properties, PartialEq)]
pub struct BackButtonProps {
    /// Whether the native back button is shown.
    #[prop_or(true)]
    pub visible:  bool,
    /// Click handler.
    #[prop_or_default]
    pub on_click: Callback<()>
}

/// Yew component for the native back button.
///
/// Shows or hides `WebApp.BackButton` based on the `visible` prop and routes
/// clicks through the `on_click` callback. The subscription is removed and the
/// button hidden on unmount.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::yew::BackButton;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let cb = Callback::from(|_| {
///         web_sys::console::log_1(&"back".into());
///     });
///     html! { <BackButton visible={true} on_click={cb} /> }
/// }
/// ```
#[function_component(BackButton)]
pub fn back_button(props: &BackButtonProps) -> Html {
    let visible = props.visible;
    let on_click = props.on_click.clone();

    use_effect_with(visible, move |&v| {
        if let Some(app) = TelegramWebApp::instance() {
            let result = if v {
                app.show_back_button()
            } else {
                app.hide_back_button()
            };
            if let Err(err) = result {
                logger::error(&format!("BackButton visibility toggle failed: {err:?}"));
            }
        }
        || ()
    });

    use_effect_with((), move |()| {
        if let Some(app) = TelegramWebApp::instance() {
            match app.set_back_button_callback(move || on_click.emit(())) {
                Ok(handle) => BACK_BUTTON_HANDLE.with(|c| {
                    *c.borrow_mut() = Some(handle);
                }),
                Err(err) => logger::error(&format!("set_back_button_callback failed: {err:?}"))
            }
        }
        || {
            if let Some(app) = TelegramWebApp::instance() {
                BACK_BUTTON_HANDLE.with(|c| {
                    if let Some(handle) = c.borrow_mut().take()
                        && let Err(err) = app.remove_back_button_callback(handle)
                    {
                        logger::error(&format!("remove_back_button_callback failed: {err:?}"));
                    }
                });
                if let Err(err) = app.hide_back_button() {
                    logger::error(&format!("hide_back_button failed: {err:?}"));
                }
            }
        }
    });

    html! {}
}
