// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

mod ui;

use telegram_webapp_sdk::{core::init::init_sdk, webapp::TelegramWebApp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    init_sdk()?;
    TelegramWebApp::instance()
        .ok_or_else(|| JsValue::from_str("Telegram WebApp not available"))?
        .ready()?;

    ui::mount();

    Ok(())
}
