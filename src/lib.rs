#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod api;
pub mod core;
pub mod logger;

#[cfg(feature = "mock")]
pub mod mock;
pub mod utils;
pub mod webapp;
#[cfg(feature = "macros")]
pub use inventory;
pub use utils::validate_init_data;
pub use webapp::TelegramWebApp;
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub mod pages;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "leptos")]
pub mod leptos;
