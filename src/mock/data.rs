// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct MockTelegramUser {
    pub id:                 u64,
    pub first_name:         String,
    pub last_name:          Option<String>,
    pub username:           Option<String>,
    pub language_code:      Option<String>,
    pub is_premium:         Option<bool>,
    pub allows_write_to_pm: Option<bool>
}
