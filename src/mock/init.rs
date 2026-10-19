// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

use super::{data::MockTelegramUser, utils::generate_mock_init_data};
use crate::{
    logger::{debug, success},
    mock::config::MockTelegramConfig
};

/// Injects a customizable mock Telegram `WebApp` environment for local
/// development.
///
/// Should be used only in `#[cfg(debug_assertions)]` environments.
///
/// # Errors
/// Returns an error when there is no browser `window` to attach the mock to, or
/// when one of the `Reflect::set` assignments on the global object is refused
/// by the JavaScript engine.
pub fn mock_telegram_webapp(config: MockTelegramConfig) -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("window not available"))?;

    let telegram = Object::new();
    let webapp = Object::new();

    // === Function mocks ===
    let init_fn = Closure::<dyn Fn()>::wrap(Box::new(|| {
        debug("WebApp.init() called");
    }));
    Reflect::set(&webapp, &"init".into(), init_fn.as_ref().unchecked_ref())?;
    init_fn.forget();

    let send_data_fn = Closure::<dyn Fn(JsValue)>::wrap(Box::new(|data: JsValue| {
        debug(&format!("WebApp.sendData(): {data:?}"));
    }));
    Reflect::set(
        &webapp,
        &"sendData".into(),
        send_data_fn.as_ref().unchecked_ref()
    )?;
    send_data_fn.forget();

    let theme = theme_params_object(&config)?;

    // === Property mocks ===
    let user = config.user.unwrap_or_else(|| MockTelegramUser {
        id: 1,
        first_name: "Dev".into(),
        ..Default::default()
    });

    let auth_date = config.auth_date.unwrap_or_else(|| "1234567890".into());
    let hash = config.hash.unwrap_or_else(|| "fakehash".into());

    let init_data = generate_mock_init_data(&user, &auth_date, &hash, config.query_id.as_deref());
    Reflect::set(&webapp, &"initData".into(), &JsValue::from_str(&init_data))?;
    Reflect::set(&webapp, &"themeParams".into(), &theme)?;

    Reflect::set(
        &webapp,
        &"platform".into(),
        &JsValue::from_str(config.platform.as_deref().unwrap_or("web"))
    )?;
    Reflect::set(
        &webapp,
        &"version".into(),
        &JsValue::from_str(config.version.as_deref().unwrap_or("9.0"))
    )?;

    Reflect::set(&telegram, &"WebApp".into(), &webapp)?;
    Reflect::set(&win, &"Telegram".into(), &telegram)?;

    // === Logs ===
    success("Mock Telegram.WebApp environment injected");

    Ok(())
}

/// Builds the `themeParams` object from the mock configuration.
///
/// Every colour falls back to the default Telegram dark palette when the
/// corresponding field of `config` is `None`.
///
/// # Errors
/// Returns an error when one of the `Reflect::set` assignments on the freshly
/// created plain object is refused by the JavaScript engine.
fn theme_params_object(config: &MockTelegramConfig) -> Result<Object, JsValue> {
    let theme = Object::new();
    let colors: [(&str, Option<&str>, &str); 15] = [
        ("bg_color", config.bg_color.as_deref(), "#17212b"),
        ("text_color", config.text_color.as_deref(), "#ffffff"),
        ("hint_color", config.hint_color.as_deref(), "#888888"),
        ("link_color", config.link_color.as_deref(), "#2689bf"),
        ("button_color", config.button_color.as_deref(), "#0088cc"),
        (
            "button_text_color",
            config.button_text_color.as_deref(),
            "#ffffff"
        ),
        (
            "secondary_bg_color",
            config.secondary_bg_color.as_deref(),
            "#f0f0f0"
        ),
        (
            "header_bg_color",
            config.header_bg_color.as_deref(),
            "#1d1f21"
        ),
        (
            "bottom_bar_bg_color",
            config.bottom_bar_bg_color.as_deref(),
            "#1f2226"
        ),
        (
            "accent_text_color",
            config.accent_text_color.as_deref(),
            "#2eaee3"
        ),
        (
            "section_bg_color",
            config.section_bg_color.as_deref(),
            "#222529"
        ),
        (
            "section_header_text_color",
            config.section_header_text_color.as_deref(),
            "#c8c9cb"
        ),
        (
            "section_separator_color",
            config.section_separator_color.as_deref(),
            "#2a2c30"
        ),
        (
            "subtitle_text_color",
            config.subtitle_text_color.as_deref(),
            "#909398"
        ),
        (
            "destructive_text_color",
            config.destructive_text_color.as_deref(),
            "#e33e3e"
        )
    ];

    for (key, value, fallback) in colors {
        let value = value.unwrap_or(fallback);
        Reflect::set(&theme, &key.into(), &JsValue::from_str(value))?;
    }

    Ok(theme)
}
