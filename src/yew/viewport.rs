// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use yew::prelude::{hook, use_effect_with, use_state};

use crate::webapp::{EventHandle, TelegramWebApp};

type HandleSlot = Rc<RefCell<Option<EventHandle<dyn FnMut()>>>>;

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

/// Yew reactive hook over `Telegram.WebApp` viewport state.
///
/// Starts with an initial snapshot and re-renders the component whenever
/// Telegram fires `viewportChanged`. The subscription is automatically
/// removed when the component unmounts.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::yew::use_viewport;
/// use yew::prelude::*;
///
/// #[component]
/// fn ViewportBadge() -> Html {
///     let viewport = use_viewport();
///     html! { <span>{ viewport.height }</span> }
/// }
/// ```
#[hook]
pub fn use_viewport() -> ViewportState {
    let state = use_state(|| ViewportState::snapshot(TelegramWebApp::instance().as_ref()));

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let stash: HandleSlot = Rc::new(RefCell::new(None));
            if let Some(app) = TelegramWebApp::instance() {
                let app_for_handler = app.clone();
                let state_for_handler = state.clone();
                if let Ok(handle) = app.on_viewport_changed(move || {
                    state_for_handler.set(ViewportState::snapshot(Some(&app_for_handler)));
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
