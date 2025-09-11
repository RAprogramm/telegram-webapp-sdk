use js_sys::{Object, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

use super::{data::MockTelegramUser, utils::generate_mock_init_data};
use crate::{
    logger::{debug, success},
    mock::config::MockTelegramConfig
};

/// Injects a customizable mock Telegram WebApp environment for local
/// development.
///
/// Should be used only in `#[cfg(debug_assertions)]` environments.
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

    // === Property mocks ===
    let user = config.user.unwrap_or_else(|| MockTelegramUser {
        id: 1,
        first_name: "Dev".into(),
        ..Default::default()
    });

    let auth_date = config.auth_date.unwrap_or_else(|| "1234567890".into());
    let hash = config.hash.unwrap_or_else(|| "fakehash".into());

    let init_data = generate_mock_init_data(&user, &auth_date, &hash);
    Reflect::set(&webapp, &"initData".into(), &JsValue::from_str(&init_data))?;

    let theme = Object::new();
    Reflect::set(
        &theme,
        &"bg_color".into(),
        &JsValue::from_str(config.bg_color.as_deref().unwrap_or("#17212b"))
    )?;
    Reflect::set(
        &theme,
        &"text_color".into(),
        &JsValue::from_str(config.text_color.as_deref().unwrap_or("#ffffff"))
    )?;
    Reflect::set(
        &theme,
        &"hint_color".into(),
        &JsValue::from_str(config.hint_color.as_deref().unwrap_or("#888888"))
    )?;
    Reflect::set(
        &theme,
        &"link_color".into(),
        &JsValue::from_str(config.link_color.as_deref().unwrap_or("#2689bf"))
    )?;
    Reflect::set(
        &theme,
        &"button_color".into(),
        &JsValue::from_str(config.button_color.as_deref().unwrap_or("#0088cc"))
    )?;
    Reflect::set(
        &theme,
        &"button_text_color".into(),
        &JsValue::from_str(config.button_text_color.as_deref().unwrap_or("#ffffff"))
    )?;
    Reflect::set(
        &theme,
        &"secondary_bg_color".into(),
        &JsValue::from_str(config.secondary_bg_color.as_deref().unwrap_or("#f0f0f0"))
    )?;
    Reflect::set(
        &theme,
        &"header_bg_color".into(),
        &JsValue::from_str(config.header_bg_color.as_deref().unwrap_or("#1d1f21"))
    )?;
    Reflect::set(
        &theme,
        &"bottom_bar_bg_color".into(),
        &JsValue::from_str(config.bottom_bar_bg_color.as_deref().unwrap_or("#1f2226"))
    )?;
    Reflect::set(
        &theme,
        &"accent_text_color".into(),
        &JsValue::from_str(config.accent_text_color.as_deref().unwrap_or("#2eaee3"))
    )?;
    Reflect::set(
        &theme,
        &"section_bg_color".into(),
        &JsValue::from_str(config.section_bg_color.as_deref().unwrap_or("#222529"))
    )?;
    Reflect::set(
        &theme,
        &"section_header_text_color".into(),
        &JsValue::from_str(
            config
                .section_header_text_color
                .as_deref()
                .unwrap_or("#c8c9cb")
        )
    )?;
    Reflect::set(
        &theme,
        &"section_separator_color".into(),
        &JsValue::from_str(
            config
                .section_separator_color
                .as_deref()
                .unwrap_or("#2a2c30")
        )
    )?;
    Reflect::set(
        &theme,
        &"subtitle_text_color".into(),
        &JsValue::from_str(config.subtitle_text_color.as_deref().unwrap_or("#909398"))
    )?;
    Reflect::set(
        &theme,
        &"destructive_text_color".into(),
        &JsValue::from_str(
            config
                .destructive_text_color
                .as_deref()
                .unwrap_or("#e33e3e")
        )
    )?;
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
