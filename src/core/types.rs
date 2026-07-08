// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

/// Chat descriptor found in the `chat` field of Telegram WebApp `initData`.
pub mod chat;
/// Parameters accepted by the `downloadFile` Telegram WebApp method.
pub mod download_file_params;
/// Parsed, strongly-typed view of the Telegram WebApp `initData` payload.
pub mod init_data;
/// Raw, string-based view of the Telegram WebApp `initData` payload used for
/// signature validation before deserialization into richer types.
pub mod init_data_internal;
/// Launch parameters read from the Mini App URL query string
/// (`tgWebApp*` parameters).
pub mod launch_params;
/// Message descriptor returned after sending data via `answerWebAppQuery`.
pub mod sent_web_app_message;
/// Telegram theme parameters exposed through `Telegram.WebApp.themeParams`.
pub mod theme_params;
/// Telegram user descriptor found in the `user` and `receiver` fields of
/// `initData`.
pub mod user;
/// Data payload delivered to the bot when the Mini App calls `sendData`.
pub mod web_app_data;
/// Metadata describing a Mini App as configured on the bot side.
pub mod web_app_info;
/// Webhook status information returned by the Telegram Bot API.
pub mod webhook_info;
/// Service message signalling that the user allowed the bot to message them.
pub mod write_access_allowed;
