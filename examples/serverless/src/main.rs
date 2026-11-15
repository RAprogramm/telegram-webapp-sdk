// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Serverless demo: a Mini App whose backend runs on Telegram's own
//! Serverless platform instead of a self-hosted server.
//!
//! The frontend is this crate (Rust + `telegram-webapp-sdk`); the backend is
//! the sibling `tgcloud/` folder (JavaScript modules deployed with the
//! `tgcloud` CLI). One `npm run deploy` ships both: the built frontend is
//! served at `https://app<app_id>.tgcloud.ai/` and `tgcloud/endpoints/`
//! answers `Telegram.WebApp.Serverless.call` through the bridge this SDK
//! wraps.
//!
//! The flow:
//!
//! 1. Mock fallback for plain-browser development (the Serverless call itself
//!    then fails with a typed
//!    [`EndpointError`](telegram_webapp_sdk::api::serverless::EndpointError),
//!    which is also part of the demo).
//! 2. `init_sdk()` + `ready()` + `expand()`.
//! 3. MainButton click calls the `getVisits` endpoint, which increments and
//!    returns a per-user counter stored in the platform database.
//!
//! See `README.md` in this directory for the end-to-end setup with your own
//! bot.

#![no_main]

use serde::{Deserialize, Serialize};
use telegram_webapp_sdk::{
    api::serverless,
    core::{context::TelegramContext, init::init_sdk},
    mock::{config::MockTelegramConfig, init::mock_telegram_webapp},
    webapp::{BottomButton, TelegramWebApp}
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

/// Empty input object for endpoints that take no parameters.
#[derive(Serialize)]
struct NoInput {}

/// Result returned by the `getVisits` endpoint.
#[derive(Deserialize)]
struct Visits {
    count: i64
}

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
            .map(|user| format!("Hello, {}!", user.first_name))
    })
    .flatten()
    .unwrap_or_else(|| "Hello, guest!".to_owned());
    set_text("greeting", &greeting)
}

/// Labels the MainButton and attaches the Serverless call.
///
/// The returned [`EventHandle`] is intentionally leaked: the callback must
/// outlive this function, and dropping the handle would unregister it.
///
/// # Errors
/// Returns a `JsValue` error when the button calls fail.
fn wire_main_button(app: &TelegramWebApp) -> Result<(), JsValue> {
    app.set_bottom_button_text(BottomButton::Main, "Call getVisits")?;
    app.show_bottom_button(BottomButton::Main)?;
    let handle = app.set_bottom_button_callback(BottomButton::Main, || {
        spawn_local(async {
            let report = call_get_visits().await;
            let _ = set_text("status", &report);
        });
    })?;
    std::mem::forget(handle);
    Ok(())
}

/// Calls the `getVisits` Serverless endpoint and renders the outcome as
/// text. Failures keep the `EndpointError` `Display`, which already
/// carries a recovery hint (for example, running outside Telegram).
async fn call_get_visits() -> String {
    match serverless::call::<NoInput, Visits>("getVisits", &NoInput {}).await {
        Ok(visits) => format!(
            "Serverless counter: this Mini App was opened {} time(s) by you.",
            visits.count
        ),
        Err(error) => format!("Serverless call failed: {error}")
    }
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
