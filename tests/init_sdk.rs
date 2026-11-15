// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![cfg(target_arch = "wasm32")]

//! `init_sdk` flow tests. The global [`TelegramContext`] is a `OnceCell` and
//! `wasm-bindgen-test` runs tests in unspecified order, so exactly one test
//! owns the full context lifecycle (uninitialized → initialized →
//! already-initialized) and no other test touches the global context.

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::{
    TelegramWebApp,
    core::{
        context::TelegramContext,
        init::{InitError, init_sdk}
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

const RICH_INIT_DATA: &str = "query_id=AAHdF6IQAAAAAN0XohDhrOrc&chat_join_request_query_id=join-777&user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Test%22%7D&start_param=hello%2Bworld+test&auth_date=1702234567&hash=f5a9d8c7b6e4a3d2c1b0a9f8e7d6c5b4";

#[wasm_bindgen_test]
fn init_sdk_context_lifecycle() -> Result<(), JsValue> {
    let uninitialized = TelegramContext::get_raw_init_data();
    assert!(uninitialized.is_err());
    assert_eq!(
        uninitialized.unwrap_err(),
        "TelegramContext not initialized"
    );

    let uninitialized_webapp = TelegramWebApp::get_raw_init_data();
    assert!(uninitialized_webapp.is_err());
    assert_eq!(
        uninitialized_webapp.unwrap_err(),
        "TelegramContext not initialized"
    );

    install_webapp(RICH_INIT_DATA)?;
    init_sdk()?;

    let query_id = TelegramContext::get(|ctx| ctx.init_data.query_id.clone())
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;
    assert_eq!(query_id, Some("AAHdF6IQAAAAAN0XohDhrOrc".to_string()));

    let join_query_id =
        TelegramContext::get(|ctx| ctx.init_data.chat_join_request_query_id.clone())
            .ok_or_else(|| JsValue::from_str("context not initialized"))?;
    assert_eq!(join_query_id, Some("join-777".to_string()));

    let user_id = TelegramContext::get(|ctx| ctx.init_data.user.as_ref().map(|user| user.id))
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;
    assert_eq!(user_id, Some(279058397));

    let start_param = TelegramContext::get(|ctx| ctx.init_data.start_param.clone())
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;
    assert_eq!(start_param, Some("hello+world test".to_string()));

    let raw = TelegramContext::get_raw_init_data().map_err(JsValue::from_str)?;
    assert_eq!(raw, RICH_INIT_DATA);

    let webapp_raw = TelegramWebApp::get_raw_init_data().map_err(JsValue::from_str)?;
    assert_eq!(webapp_raw, RICH_INIT_DATA);
    assert!(webapp_raw.contains("auth_date="));
    assert!(webapp_raw.contains("hash="));
    assert!(webapp_raw.contains("query_id="));

    let err = init_sdk().expect_err("second init must fail");
    assert!(
        err.as_string()
            .unwrap_or_default()
            .contains("already initialized")
    );

    Ok(())
}

#[wasm_bindgen_test]
fn init_error_display_formatting() {
    let window = InitError::WindowUnavailable.to_string();
    assert!(window.starts_with("Browser window object is not available"));
    assert!(window.contains("wasm32-unknown-unknown"));

    let telegram = InitError::TelegramUnavailable.to_string();
    assert!(telegram.starts_with("window.Telegram is undefined"));
    assert!(telegram.contains("mock_telegram_webapp"));

    let webapp = InitError::WebAppUnavailable.to_string();
    assert!(webapp.starts_with("Telegram.WebApp is undefined"));
    assert!(webapp.contains("telegram-web-app.js"));

    let init_data = InitError::InitDataParseFailed("test error".to_string()).to_string();
    assert!(init_data.contains("Failed to parse initData: test error"));
    assert!(init_data.contains("URL-encoded"));

    let theme = InitError::ThemeParamsParseFailed("theme error".to_string()).to_string();
    assert!(theme.contains("Failed to parse theme parameters: theme error"));
    assert!(theme.contains("themeParams"));

    let context = InitError::ContextInitFailed("context error".to_string()).to_string();
    assert!(context.contains("Failed to initialize context: context error"));
    assert!(context.contains("once per page"));
}

#[wasm_bindgen_test]
fn init_error_converts_to_jsvalue() {
    let err = InitError::WindowUnavailable;
    let js_val: JsValue = err.into();

    assert!(
        js_val
            .as_string()
            .unwrap_or_default()
            .starts_with("Browser window object is not available")
    );
}
