// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::{
    TelegramWebApp,
    core::{context::TelegramContext, init::init_sdk}
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

#[wasm_bindgen_test]
fn init_sdk_propagates_query_id() -> Result<(), JsValue> {
    install_webapp("query_id=inline-123&auth_date=1&hash=abc")?;

    init_sdk()?;

    let query_id = TelegramContext::get(|ctx| ctx.init_data.query_id.clone())
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;

    assert_eq!(query_id, Some("inline-123".to_string()));

    Ok(())
}

#[wasm_bindgen_test]
fn get_raw_init_data_returns_error_when_not_initialized() {
    let result = TelegramContext::get_raw_init_data();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "TelegramContext not initialized");
}

#[wasm_bindgen_test]
fn get_raw_init_data_returns_original_string() -> Result<(), JsValue> {
    let expected = "query_id=test123&auth_date=1234567890&hash=abcdef";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramContext::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn get_raw_init_data_preserves_url_encoding() -> Result<(), JsValue> {
    let expected = "user=%7B%22id%22%3A123%7D&auth_date=1&hash=xyz";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramContext::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn get_raw_init_data_handles_complex_payload() -> Result<(), JsValue> {
    let expected = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Test%22%2C%22last_name%22%3A%22User%22%2C%22username%22%3A%22testuser%22%2C%22language_code%22%3A%22en%22%7D&auth_date=1702234567&hash=f5a9d8c7b6e4a3d2c1b0a9f8e7d6c5b4a3e2d1c0b9a8f7e6d5c4b3a2e1d0c9b8";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramContext::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn get_raw_init_data_handles_minimal_payload() -> Result<(), JsValue> {
    let expected = "auth_date=1&hash=x";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramContext::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn get_raw_init_data_handles_special_characters() -> Result<(), JsValue> {
    let expected = "start_param=hello%2Bworld+test&auth_date=1&hash=abc123";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramContext::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn webapp_get_raw_init_data_returns_error_when_not_initialized() {
    let result = TelegramWebApp::get_raw_init_data();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "TelegramContext not initialized");
}

#[wasm_bindgen_test]
fn webapp_get_raw_init_data_returns_original_string() -> Result<(), JsValue> {
    let expected = "query_id=webapp_test&auth_date=9999999999&hash=fedcba";
    install_webapp(expected)?;

    init_sdk()?;

    let raw = TelegramWebApp::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert_eq!(raw, expected);

    Ok(())
}

#[wasm_bindgen_test]
fn webapp_get_raw_init_data_suitable_for_validation() -> Result<(), JsValue> {
    let init_data = "auth_date=1234567890&hash=test_hash_value&query_id=test_query";
    install_webapp(init_data)?;

    init_sdk()?;

    let raw = TelegramWebApp::get_raw_init_data().map_err(|e| JsValue::from_str(e))?;

    assert!(raw.contains("auth_date="));
    assert!(raw.contains("hash="));
    assert!(raw.contains("query_id="));

    Ok(())
}
