// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect};
use leptos::prelude::use_context;
use telegram_webapp_sdk::{
    core::{context::TelegramContext, init::init_sdk},
    leptos::provide_telegram_context
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
fn provide_telegram_context_succeeds_after_init() -> Result<(), JsValue> {
    install_webapp("query_id=test&auth_date=1&hash=abc")?;
    init_sdk()?;

    leptos::prelude::Owner::new().with(|| {
        provide_telegram_context()?;
        let ctx = use_context::<TelegramContext>()
            .ok_or_else(|| JsValue::from_str("context not provided"))?;

        assert_eq!(ctx.init_data.query_id.as_deref(), Some("test"));
        Ok(())
    })
}

#[wasm_bindgen_test]
fn provide_telegram_context_fails_without_init() {
    leptos::prelude::Owner::new().with(|| {
        let result = provide_telegram_context();
        assert!(result.is_err());
    })
}
