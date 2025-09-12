#![no_main]

pub mod components;
pub mod pages;
pub mod router;

use telegram_webapp_sdk::{telegram_app, telegram_router};
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
use components::dev_menu::setup_dev_menu;
#[rustfmt::skip]
use router::Router;

telegram_app!(
    pub fn main() -> Result<(), JsValue> {
        setup_dev_menu();
        telegram_router!();
        Ok(())
    }
);
