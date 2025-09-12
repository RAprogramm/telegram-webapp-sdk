#![no_main]

pub mod components;
pub mod pages;
pub mod router;

use components::dev_menu::setup_dev_menu;
use router::Router;
use telegram_webapp_sdk::{telegram_app, telegram_router};
use wasm_bindgen::prelude::*;

telegram_app!(
    pub fn main() -> Result<(), JsValue> {
        setup_dev_menu();
        telegram_router!();
        Ok(())
    }
);
