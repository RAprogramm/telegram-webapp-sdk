// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![cfg(target_arch = "wasm32")]

//! `try_init_sdk` and environment detection tests. Kept in a separate test
//! binary so the global [`TelegramContext`] starts uninitialized. Exactly one
//! test owns the context lifecycle because `wasm-bindgen-test` runs tests in
//! unspecified order.

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::{
    TelegramWebApp,
    core::{
        context::TelegramContext,
        init::{init_sdk, is_telegram_available, try_init_sdk}
    }
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::window;

wasm_bindgen_test_configure!(run_in_browser);

fn install_webapp(init_data: &str) -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let telegram = Object::new();
    let webapp = Object::new();

    Reflect::set(&webapp, &"initData".into(), &JsValue::from_str(init_data))?;
    Reflect::set(
        &webapp,
        &"themeParams".into(),
        &JsValue::from(Object::new())
    )?;

    Reflect::set(&telegram, &"WebApp".into(), &webapp)?;
    Reflect::set(&win, &"Telegram".into(), &telegram)?;

    Ok(())
}

fn remove_telegram() -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    Reflect::delete_property(&win, &"Telegram".into())?;
    Ok(())
}

#[wasm_bindgen_test]
fn try_init_sdk_context_lifecycle() -> Result<(), JsValue> {
    let minimal = "auth_date=1&hash=x";
    install_webapp(minimal)?;

    assert!(try_init_sdk().map_err(|e| JsValue::from_str(&e.to_string()))?);

    let raw = TelegramWebApp::get_raw_init_data().map_err(JsValue::from_str)?;
    assert_eq!(raw, minimal);

    let query_id = TelegramContext::get(|ctx| ctx.init_data.query_id.clone())
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;
    assert_eq!(query_id, None);

    let err = init_sdk().expect_err("second init must fail");
    assert!(
        err.as_string()
            .unwrap_or_default()
            .contains("already initialized")
    );

    Ok(())
}

#[wasm_bindgen_test]
fn is_telegram_available_returns_true_when_installed() -> Result<(), JsValue> {
    install_webapp("auth_date=1&hash=abc")?;

    assert!(is_telegram_available());

    Ok(())
}

#[wasm_bindgen_test]
fn is_telegram_available_returns_false_when_not_installed() -> Result<(), JsValue> {
    remove_telegram()?;

    assert!(!is_telegram_available());

    Ok(())
}

#[wasm_bindgen_test]
fn try_init_sdk_returns_false_when_telegram_unavailable() -> Result<(), JsValue> {
    remove_telegram()?;

    assert!(!try_init_sdk().map_err(|e| JsValue::from_str(&e.to_string()))?);

    Ok(())
}
