// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Golden-path Mini App: the shortest complete journey from a cold page load
//! to a working Telegram Mini App, used as the canonical starting point.
//!
//! The flow it demonstrates:
//!
//! 1. Fall back to the SDK mock when `Telegram.WebApp` is absent, so the same
//!    binary runs in a plain browser during development.
//! 2. `init_sdk()` — parse `initData` and `themeParams` into the global
//!    [`TelegramContext`].
//! 3. `ready()` + `expand()` — signal Telegram that the interface is up.
//! 4. Greet the launching user from the parsed context.
//! 5. Wire the MainButton: each click fires haptic feedback and persists a
//!    visit counter through `DeviceStorage`.
//!
//! Inside Telegram the platform injects `telegram-web-app.js` itself; the app
//! deliberately does not include the script tag so that outside Telegram the
//! mock path is taken instead of a half-initialized real bridge.
//!
//! Run with [`trunk`](https://trunkrs.dev):
//!
//! ```sh
//! cd examples/minimal
//! trunk serve        # http://localhost:8081
//! ```

#![no_main]

use telegram_webapp_sdk::{
    api::{
        device_storage,
        haptic::{HapticImpactStyle, impact_occurred}
    },
    core::{context::TelegramContext, init::init_sdk},
    mock::{config::MockTelegramConfig, init::mock_telegram_webapp},
    webapp::{BottomButton, TelegramWebApp}
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

/// `DeviceStorage` key holding the visit counter.
const VISITS_KEY: &str = "minimal_visits";

/// Boots the mini app: installs the mock when needed, initializes the SDK,
/// marks the interface ready, renders the greeting and wires the MainButton.
///
/// # Errors
/// Returns a `JsValue` error when the mock cannot be installed, `init_sdk`
/// cannot parse the launch context, the WebApp handle is unavailable, or a
/// bridge call (`ready`, `expand`, button wiring) throws.
#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    if TelegramWebApp::instance().is_none() {
        mock_telegram_webapp(MockTelegramConfig::default())?;
    }
    init_sdk()?;
    let app = TelegramWebApp::try_instance()?;
    app.ready()?;
    app.expand()?;
    render_greeting()?;
    wire_main_button(&app)?;
    Ok(())
}

/// Writes a greeting for the launching user into the `#greeting` element,
/// falling back to a generic greeting when the launch carries no user.
///
/// # Errors
/// Returns a `JsValue` error when the document or the target element is
/// missing from the page.
fn render_greeting() -> Result<(), JsValue> {
    let greeting = TelegramContext::get(|context| {
        context
            .init_data
            .user
            .as_ref()
            .map(|user| format!("Hello, {}! Your launch is validated.", user.first_name))
    })
    .flatten()
    .unwrap_or_else(|| "Hello, guest!".to_owned());
    set_text("greeting", &greeting)
}

/// Labels the MainButton and attaches the visit-recording callback.
///
/// The returned [`EventHandle`] is intentionally leaked: the callback must
/// outlive this function, and dropping the handle would unregister it.
///
/// # Errors
/// Returns a `JsValue` error when the button calls fail.
fn wire_main_button(app: &TelegramWebApp) -> Result<(), JsValue> {
    app.set_bottom_button_text(BottomButton::Main, "Record a visit")?;
    app.show_bottom_button(BottomButton::Main)?;
    let handle = app.set_bottom_button_callback(BottomButton::Main, || {
        spawn_local(async {
            let report = match record_visit().await {
                Ok(text) => text,
                Err(error) => format!("Storage failed: {error:?}")
            };
            let _ = set_text("visits", &report);
        });
    })?;
    std::mem::forget(handle);
    Ok(())
}

/// Increments the visit counter persisted in `DeviceStorage`, fires light
/// haptic feedback and returns the text describing the new counter value.
///
/// # Errors
/// Returns a `JsValue` error when the storage roundtrip fails.
async fn record_visit() -> Result<String, JsValue> {
    let previous = device_storage::get_item(VISITS_KEY).await?;
    let visits = previous
        .and_then(|raw| raw.parse::<u32>().ok())
        .unwrap_or(0)
        + 1;
    device_storage::set_item(VISITS_KEY, &visits.to_string()).await?;
    let _ = impact_occurred(HapticImpactStyle::Light);
    Ok(format!("Visits stored in DeviceStorage: {visits}"))
}

/// Sets the text content of the element with the given id.
///
/// # Errors
/// Returns a `JsValue` error when the document or the element is missing.
fn set_text(id: &str, text: &str) -> Result<(), JsValue> {
    let document = window()
        .and_then(|win| win.document())
        .ok_or_else(|| JsValue::from_str("no document on this page"))?;
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("missing element"))?;
    element.set_text_content(Some(text));
    Ok(())
}
