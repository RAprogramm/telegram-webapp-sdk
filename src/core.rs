// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

/// Global [`context::TelegramContext`] holding parsed init data, theme
/// parameters and the raw init-data string for the current Mini App session.
pub mod context;
/// SDK initialization routines that populate the global context from the
/// running Telegram WebApp environment.
pub mod init;
/// Fallible accessors for the global context that return a
/// [`wasm_bindgen::JsValue`] error instead of an [`Option`] when the context is
/// not initialized.
pub mod safe_context;
/// Strongly-typed representations of the Telegram WebApp `initData`,
/// launch parameters, theme parameters and related payload structures.
pub mod types;
