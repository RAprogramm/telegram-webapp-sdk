#![no_main]

pub mod components;
pub mod pages;
pub mod router;

<<<<<<< HEAD
use components::dev_menu::setup_dev_menu;
use router::Router;
use telegram_webapp_sdk::{telegram_app, telegram_router};
||||||| 8d67f76
use components::dev_menu::setup_dev_menu;
use router::Router;
=======
use telegram_webapp_sdk::{telegram_app, telegram_router};
>>>>>>> 2b4e52276e4376d87fb7fc4b5147e589d12258fe
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
