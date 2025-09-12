#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod api;
pub mod core;
pub mod logger;

#[cfg(feature = "mock")]
pub mod mock;
pub mod utils;
pub mod webapp;
pub use utils::validate_init_data;
pub use webapp::TelegramWebApp;
#[cfg(feature = "macros")]
pub mod pages;

#[cfg(feature = "macros")]
pub use telegram_webapp_sdk_macros::{telegram_app, telegram_page, telegram_router};

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "leptos")]
pub mod leptos;
