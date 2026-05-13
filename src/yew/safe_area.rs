// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use yew::prelude::{hook, use_effect_with, use_state};

use crate::webapp::{EventHandle, SafeAreaInset, TelegramWebApp};

type HandleList = Rc<RefCell<Vec<EventHandle<dyn FnMut()>>>>;

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

/// Yew reactive hook over the safe-area insets.
///
/// Updates on both `safeAreaChanged` and `contentSafeAreaChanged`. The
/// subscriptions are removed on unmount.
#[hook]
pub fn use_safe_area() -> SafeAreaState {
    let state = use_state(|| SafeAreaState::snapshot(TelegramWebApp::instance().as_ref()));

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let stash: HandleList = Rc::new(RefCell::new(Vec::new()));
            if let Some(app) = TelegramWebApp::instance() {
                {
                    let app_for_handler = app.clone();
                    let state_for_handler = state.clone();
                    if let Ok(handle) = app.on_safe_area_changed(move || {
                        state_for_handler.set(SafeAreaState::snapshot(Some(&app_for_handler)));
                    }) {
                        stash.borrow_mut().push(handle);
                    }
                }
                {
                    let app_for_handler = app.clone();
                    let state_for_handler = state.clone();
                    if let Ok(handle) = app.on_content_safe_area_changed(move || {
                        state_for_handler.set(SafeAreaState::snapshot(Some(&app_for_handler)));
                    }) {
                        stash.borrow_mut().push(handle);
                    }
                }
            }
            move || {
                stash.borrow_mut().clear();
            }
        });
    }

    (*state).clone()
}
