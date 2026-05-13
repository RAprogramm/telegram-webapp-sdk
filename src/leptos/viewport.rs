// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::webapp::TelegramWebApp;

/// Snapshot of `Telegram.WebApp`'s viewport-related properties.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ViewportState {
    /// Current visible viewport height in CSS pixels.
    pub height:        f64,
    /// Stable viewport height (does not change while the user pulls the chat).
    pub stable_height: f64,
    /// Whether the mini app is currently expanded.
    pub is_expanded:   bool
}

impl ViewportState {
    fn snapshot(app: Option<&TelegramWebApp>) -> Self {
        match app {
            Some(app) => Self {
                height:        app.viewport_height().unwrap_or(0.0),
                stable_height: app.viewport_stable_height().unwrap_or(0.0),
                is_expanded:   app.is_expanded()
            },
            None => Self::default()
        }
    }
}

/// Leptos reactive hook over `Telegram.WebApp` viewport state.
///
/// The returned [`ReadSignal`] starts with a snapshot taken at mount time and
/// updates whenever Telegram fires `viewportChanged`. The underlying event
/// subscription is automatically removed when the owning Leptos scope is
/// disposed.
///
/// # Examples
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::leptos::use_viewport;
///
/// #[component]
/// fn ViewportBadge() -> impl IntoView {
///     let viewport = use_viewport();
///     view! { <span>{ move || viewport.get().height }</span> }
/// }
/// ```
pub fn use_viewport() -> ReadSignal<ViewportState> {
    let app = TelegramWebApp::instance();
    let signal = RwSignal::new(ViewportState::snapshot(app.as_ref()));

    if let Some(app) = app {
        let app_for_handler = app.clone();
        let writer = signal;
        if let Ok(handle) = app.on_viewport_changed(move || {
            writer.set(ViewportState::snapshot(Some(&app_for_handler)));
        }) {
            let wrapped = SendWrapper::new(handle);
            on_cleanup(move || {
                drop(wrapped);
            });
        }
    }

    signal.read_only()
}
