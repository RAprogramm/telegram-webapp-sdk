// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use telegram_webapp_sdk::{core::init::init_sdk, webapp::TelegramWebApp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    init_sdk()?;

    let webapp = TelegramWebApp::instance()
        .ok_or_else(|| JsValue::from_str("Telegram WebApp not available"))?;
    webapp.ready()?;

    console::log_1(&"WebApp Integration Demo initialized".into());

    Ok(())
}
