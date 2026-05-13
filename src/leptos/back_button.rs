// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::cell::RefCell;

use leptos::prelude::*;

use crate::{
    logger,
    webapp::{EventHandle, TelegramWebApp}
};

thread_local! {
    static BACK_BUTTON_HANDLE: RefCell<Option<EventHandle<dyn FnMut()>>> =
        const { RefCell::new(None) };
}

/// Leptos component for the native back button.
///
/// Mirrors the React SDK's `BackButton` ergonomics. Drives `WebApp.BackButton`:
/// shows/hides it based on the `visible` signal, registers the optional click
/// callback, and cleans both up on unmount.
///
/// # Examples
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::leptos::BackButton;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let visible = RwSignal::new(true);
///     view! {
///         <BackButton
///             visible=visible
///             on_click=move || { web_sys::console::log_1(&"back".into()); }
///         />
///     }
/// }
/// ```
#[component]
pub fn BackButton<F>(
    #[prop(into)] visible: Signal<bool>,
    #[prop(optional)] on_click: Option<F>
) -> impl IntoView
where
    F: Fn() + Clone + 'static
{
    Effect::new(move |_| {
        if let Some(app) = TelegramWebApp::instance() {
            let result = if visible.get() {
                app.show_back_button()
            } else {
                app.hide_back_button()
            };
            if let Err(err) = result {
                logger::error(&format!("BackButton visibility toggle failed: {err:?}"));
            }
        }
    });

    if let Some(cb) = on_click
        && let Some(app) = TelegramWebApp::instance()
    {
        match app.set_back_button_callback(cb) {
            Ok(handle) => BACK_BUTTON_HANDLE.with(|c| {
                *c.borrow_mut() = Some(handle);
            }),
            Err(err) => logger::error(&format!("set_back_button_callback failed: {err:?}"))
        }
    }

    on_cleanup(move || {
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
    });

    View::new(())
}
