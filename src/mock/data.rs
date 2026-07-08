// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Mocked Telegram user, mirroring the Telegram WebApp `WebAppUser` object.
#[derive(Serialize, Deserialize, Default)]
pub struct MockTelegramUser {
    /// Unique identifier of the user.
    pub id:                 u64,
    /// First name of the user.
    pub first_name:         String,
    /// Last name of the user, if set.
    pub last_name:          Option<String>,
    /// Telegram username, without the leading `@`, if set.
    pub username:           Option<String>,
    /// IETF language tag of the user's language (e.g. `en`), if known.
    pub language_code:      Option<String>,
    /// Whether the user has a Telegram Premium subscription.
    pub is_premium:         Option<bool>,
    /// Whether the user allowed the bot to message them in private chat.
    pub allows_write_to_pm: Option<bool>
}
