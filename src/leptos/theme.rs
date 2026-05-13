// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::{
    api::theme::get_theme_params, core::types::theme_params::TelegramThemeParams,
    webapp::TelegramWebApp
};

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

/// Leptos reactive hook over `Telegram.WebApp` theme state.
///
/// Updates on `themeChanged`. The subscription is removed on scope disposal.
///
/// # Examples
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::leptos::use_theme;
///
/// #[component]
/// fn ThemeBadge() -> impl IntoView {
///     let theme = use_theme();
///     view! { <span>{ move || theme.get().color_scheme.unwrap_or_default() }</span> }
/// }
/// ```
pub fn use_theme() -> ReadSignal<ThemeState> {
    let app = TelegramWebApp::instance();
    let signal = RwSignal::new(ThemeState::snapshot(app.as_ref()));

    if let Some(app) = app {
        let app_for_handler = app.clone();
        let writer = signal;
        if let Ok(handle) = app.on_theme_changed(move || {
            writer.set(ThemeState::snapshot(Some(&app_for_handler)));
        }) {
            let wrapped = SendWrapper::new(handle);
            on_cleanup(move || {
                drop(wrapped);
            });
        }
    }

    signal.read_only()
}
