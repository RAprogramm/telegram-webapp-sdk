// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]
#![cfg_attr(all(docsrs, has_doc_cfg), feature(doc_cfg))]
#![cfg_attr(all(docsrs, not(has_doc_cfg), has_doc_auto_cfg), feature(doc_auto_cfg))]

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
#[cfg(feature = "macros")]
#[allow(unused_imports)]
pub use crate::macros::*;
pub mod router;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "leptos")]
pub mod leptos;

/// Captures code coverage data for WASM builds.
///
/// This function is used by wasmcov to collect coverage information during test
/// execution. It should not be called directly in application code.
///
/// # Safety
///
/// This function is marked as unsafe due to FFI requirements.
/// It is only exported for wasm32 targets and called by the wasmcov test
/// harness.
#[cfg(all(target_family = "wasm", test))]
#[no_mangle]
pub unsafe extern "C" fn capture_coverage() {
    let mut coverage = Vec::new();
    if let Err(e) = wasmcov::minicov::capture_coverage(&mut coverage) {
        web_sys::console::error_1(&format!("Coverage capture failed: {}", e).into());
    }
}
