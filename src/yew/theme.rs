// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use yew::prelude::{hook, use_effect_with, use_state};

use crate::{
    api::theme::get_theme_params,
    core::types::theme_params::TelegramThemeParams,
    webapp::{EventHandle, TelegramWebApp}
};

type HandleSlot = Rc<RefCell<Option<EventHandle<dyn FnMut()>>>>;

/// Snapshot of `Telegram.WebApp` theme state.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeState {
    /// `"light"` or `"dark"`.
    pub color_scheme: Option<String>,
    /// Parsed theme palette.
    pub params:       TelegramThemeParams
}

impl ThemeState {
    fn snapshot(app: Option<&TelegramWebApp>) -> Self {
        let color_scheme = app.and_then(|a| a.color_scheme());
        let params = get_theme_params().unwrap_or_default();
        Self {
            color_scheme,
            params
        }
    }
}

/// Yew reactive hook over `Telegram.WebApp` theme state.
///
/// Updates on `themeChanged`. The subscription is removed on unmount.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::yew::use_theme;
/// use yew::prelude::*;
///
/// #[component]
/// fn ThemeBadge() -> Html {
///     let theme = use_theme();
///     html! { <span>{ theme.color_scheme.clone().unwrap_or_default() }</span> }
/// }
/// ```
#[hook]
pub fn use_theme() -> ThemeState {
    let state = use_state(|| ThemeState::snapshot(TelegramWebApp::instance().as_ref()));

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let stash: HandleSlot = Rc::new(RefCell::new(None));
            if let Some(app) = TelegramWebApp::instance() {
                let app_for_handler = app.clone();
                let state_for_handler = state.clone();
                if let Ok(handle) = app.on_theme_changed(move || {
                    state_for_handler.set(ThemeState::snapshot(Some(&app_for_handler)));
                }) {
                    *stash.borrow_mut() = Some(handle);
                }
            }
            move || {
                stash.borrow_mut().take();
            }
        });
    }

    (*state).clone()
}
