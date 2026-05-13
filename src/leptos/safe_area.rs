// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::webapp::{EventHandle, SafeAreaInset, TelegramWebApp};

/// Snapshot of `Telegram.WebApp` safe-area insets.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SafeAreaState {
    /// `WebApp.safeAreaInset`.
    pub area:    Option<SafeAreaInset>,
    /// `WebApp.contentSafeAreaInset`.
    pub content: Option<SafeAreaInset>
}

impl SafeAreaState {
    fn snapshot(app: Option<&TelegramWebApp>) -> Self {
        match app {
            Some(app) => Self {
                area:    app.safe_area_inset(),
                content: app.content_safe_area_inset()
            },
            None => Self::default()
        }
    }
}

/// Leptos reactive hook over the safe-area insets.
///
/// Updates on both `safeAreaChanged` and `contentSafeAreaChanged`. The
/// subscriptions are removed on scope disposal.
///
/// # Examples
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::leptos::use_safe_area;
///
/// #[component]
/// fn TopPadding() -> impl IntoView {
///     let safe = use_safe_area();
///     view! { <div>{ move || safe.get().area.map(|i| i.top).unwrap_or(0.0) }</div> }
/// }
/// ```
pub fn use_safe_area() -> ReadSignal<SafeAreaState> {
    let app = TelegramWebApp::instance();
    let signal = RwSignal::new(SafeAreaState::snapshot(app.as_ref()));

    if let Some(app) = app {
        let writer = signal;

        let app_area = app.clone();
        let area_handle: Option<EventHandle<dyn FnMut()>> = app
            .on_safe_area_changed(move || {
                writer.set(SafeAreaState::snapshot(Some(&app_area)));
            })
            .ok();

        let app_content = app.clone();
        let content_handle: Option<EventHandle<dyn FnMut()>> = app
            .on_content_safe_area_changed(move || {
                writer.set(SafeAreaState::snapshot(Some(&app_content)));
            })
            .ok();

        let wrapped = SendWrapper::new((area_handle, content_handle));
        on_cleanup(move || {
            drop(wrapped);
        });
    }

    signal.read_only()
}
