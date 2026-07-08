// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::JsValue;

use crate::core::context::TelegramContext;

/// Accesses the global [`TelegramContext`] and applies `f` to it.
///
/// This is the fallible counterpart to [`TelegramContext::get`]: instead of
/// returning [`None`] when the context has not been initialized, it returns a
/// [`JsValue`] error suitable for propagation across the WASM boundary.
///
/// # Errors
///
/// Returns `Err(JsValue)` if the global context has not been initialized via
/// [`crate::core::init::init_sdk`].
pub fn get_context<T>(f: impl FnOnce(&TelegramContext) -> T) -> Result<T, JsValue> {
    TelegramContext::get(f).ok_or_else(|| JsValue::from_str("TelegramContext is not initialized"))
}
