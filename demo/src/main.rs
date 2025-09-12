#![no_main]

pub mod components;
pub mod pages;
pub mod router;

use components::dev_menu::setup_dev_menu;
use pages::{
    burger_king::render_burger_king_page, index::render_index_page,
    init_data::render_init_data_page, launch_params::render_launch_params_page,
    theme_params::render_theme_params_page,
};
use router::Router;
use telegram_webapp_sdk::{
    core::init::init_sdk,
    mock::{config::MockTelegramConfig, data::MockTelegramUser, init::mock_telegram_webapp},
    utils::check_env::is_telegram_env,
};
use wasm_bindgen::prelude::*;

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
            hint_color: Some("#888888".into()),
            link_color: Some("#2689bf".into()),
            button_color: Some("#0088cc".into()),
            button_text_color: Some("#ffffff".into()),
            secondary_bg_color: Some("#f0f0f0".into()),
            header_bg_color: Some("#1d1f21".into()),
            bottom_bar_bg_color: Some("#1f2226".into()),
            accent_text_color: Some("#2eaee3".into()),
            section_bg_color: Some("#222529".into()),
            section_header_text_color: Some("#c8c9cb".into()),
            section_separator_color: Some("#2a2c30".into()),
            subtitle_text_color: Some("#909398".into()),
            destructive_text_color: Some("#e33e3e".into()),
            ..Default::default()
        })
        .unwrap();
    }

    init_sdk()?;

    setup_dev_menu();

    Router::new()
        .register("/", render_index_page)
        .register("/init-data", render_init_data_page)
        .register("/launch-params", render_launch_params_page)
        .register("/theme-params", render_theme_params_page)
        .register("/burger-king", render_burger_king_page)
        .start();

    Ok(())
}
