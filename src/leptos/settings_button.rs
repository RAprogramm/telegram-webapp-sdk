// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::cell::RefCell;

use leptos::prelude::*;

use crate::{
    logger,
    webapp::{EventHandle, TelegramWebApp}
};

thread_local! {
    static SETTINGS_BUTTON_HANDLE: RefCell<Option<EventHandle<dyn FnMut()>>> =
        const { RefCell::new(None) };
}

/// Leptos component for the native settings button.
///
/// Drives `WebApp.SettingsButton`: shows/hides it based on the `visible`
/// signal, registers the optional click callback, and cleans both up on
/// unmount.
///
/// # Examples
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::leptos::SettingsButton;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let visible = RwSignal::new(true);
///     view! {
///         <SettingsButton
///             visible=visible
///             on_click=move || { web_sys::console::log_1(&"settings".into()); }
///         />
///     }
/// }
/// ```
#[component]
pub fn SettingsButton<F>(
    #[prop(into)] visible: Signal<bool>,
    #[prop(optional)] on_click: Option<F>
) -> impl IntoView
where
    F: Fn() + Clone + 'static
{
    Effect::new(move |_| {
        if let Some(app) = TelegramWebApp::instance() {
            let result = if visible.get() {
                app.show_settings_button()
            } else {
                app.hide_settings_button()
            };
            if let Err(err) = result {
                logger::error(&format!("SettingsButton visibility toggle failed: {err:?}"));
            }
        }
    });

    if let Some(cb) = on_click
        && let Some(app) = TelegramWebApp::instance()
    {
        match app.set_settings_button_callback(cb) {
            Ok(handle) => SETTINGS_BUTTON_HANDLE.with(|c| {
                *c.borrow_mut() = Some(handle);
            }),
            Err(err) => logger::error(&format!("set_settings_button_callback failed: {err:?}"))
        }
    }

    on_cleanup(move || {
        if let Some(app) = TelegramWebApp::instance() {
            SETTINGS_BUTTON_HANDLE.with(|c| {
                if let Some(handle) = c.borrow_mut().take()
                    && let Err(err) = app.remove_settings_button_callback(handle)
                {
                    logger::error(&format!("remove_settings_button_callback failed: {err:?}"));
                }
            });
            if let Err(err) = app.hide_settings_button() {
                logger::error(&format!("hide_settings_button failed: {err:?}"));
            }
        }
    });

    View::new(())
}
