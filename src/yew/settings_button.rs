// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::cell::RefCell;

use yew::prelude::{Callback, Html, Properties, function_component, html, use_effect_with};

use crate::{
    logger,
    webapp::{EventHandle, TelegramWebApp}
};

thread_local! {
    static SETTINGS_BUTTON_HANDLE: RefCell<Option<EventHandle<dyn FnMut()>>> =
        const { RefCell::new(None) };
}

/// Props for [`SettingsButton`].
#[derive(Properties, PartialEq)]
pub struct SettingsButtonProps {
    /// Whether the native settings button is shown.
    #[prop_or(true)]
    pub visible:  bool,
    /// Click handler.
    #[prop_or_default]
    pub on_click: Callback<()>
}

/// Yew component for the native settings button.
///
/// Shows or hides `WebApp.SettingsButton` based on the `visible` prop and
/// routes clicks through the `on_click` callback. The subscription is removed
/// and the button hidden on unmount.
#[function_component(SettingsButton)]
pub fn settings_button(props: &SettingsButtonProps) -> Html {
    let visible = props.visible;
    let on_click = props.on_click.clone();

    use_effect_with(visible, move |&v| {
        if let Some(app) = TelegramWebApp::instance() {
            let result = if v {
                app.show_settings_button()
            } else {
                app.hide_settings_button()
            };
            if let Err(err) = result {
                logger::error(&format!("SettingsButton visibility toggle failed: {err:?}"));
            }
        }
        || ()
    });

    use_effect_with((), move |()| {
        if let Some(app) = TelegramWebApp::instance() {
            match app.set_settings_button_callback(move || on_click.emit(())) {
                Ok(handle) => SETTINGS_BUTTON_HANDLE.with(|c| {
                    *c.borrow_mut() = Some(handle);
                }),
                Err(err) => logger::error(&format!("set_settings_button_callback failed: {err:?}"))
            }
        }
        || {
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
        }
    });

    html! {}
}
