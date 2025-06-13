#![no_main]

use telegram_webapp_sdk::{
    core::{init::init_sdk, safe_context::get_context},
    logger::info,
    mock::{config::MockTelegramConfig, data::MockTelegramUser, init::mock_telegram_webapp},
    utils::check_env::is_telegram_env
};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    if !is_telegram_env() {
        #[cfg(debug_assertions)]
        mock_telegram_webapp(MockTelegramConfig {
            user: Some(MockTelegramUser {
                id: 777,
                first_name: "Alice".into(),
                username: Some("alice_dev".into()),
                is_premium: Some(true),
                ..Default::default()
            }),
            auth_date: Some("1234567890".into()),
            hash: Some("fakehash".into()),
            bg_color: Some("#ffffff".into()),
            text_color: Some("#000000".into()),
            ..Default::default()
        })
        .unwrap();
    }

    init_sdk()?;

    let lang = get_context(|ctx| {
        JsValue::from_str(
            ctx.init_data
                .user
                .as_ref()
                .and_then(|u| u.language_code.as_deref())
                .unwrap_or("en")
        )
    })?;
    info(&format!(
        "language_code: {}",
        lang.as_string().unwrap_or_default()
    ));

    let color = get_context(|ctx| {
        JsValue::from_str(ctx.theme_params.bg_color.as_deref().unwrap_or("none"))
    })?;
    info(&format!(
        "theme.bg_color: {}",
        color.as_string().unwrap_or_default()
    ));

    let user_str = get_context(|ctx| {
        ctx.init_data
            .user
            .as_ref()
            .map(|u| {
                format!(
                    "👤 {} {}\nUsername: @{}\nLanguage: {}\nID: {}",
                    u.first_name,
                    u.last_name.as_deref().unwrap_or(""),
                    u.username.as_deref().unwrap_or("–"),
                    u.language_code.as_deref().unwrap_or("en"),
                    u.id
                )
            })
            .unwrap_or_else(|| "No user info".into())
    })?;

    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Some(elem) = doc.get_element_by_id("user-info") {
            elem.set_inner_html(&user_str);
        }
    }

    info(&format!("\nUser: {user_str}"));
    Ok(())
}
