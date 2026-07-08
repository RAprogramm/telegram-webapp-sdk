// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{
    fs,
    io::{Error, ErrorKind}
};

use serde::Deserialize;

use super::data::MockTelegramUser;

/// Configuration for mocking Telegram environment.
#[derive(Default, Deserialize)]
pub struct MockTelegramConfig {
    /// Mocked Telegram user to expose through `initDataUnsafe`.
    pub user: Option<MockTelegramUser>,
    /// Unix timestamp (as a string) when the init data was authorized.
    pub auth_date: Option<String>,
    /// Hash used to verify the authenticity of the init data.
    pub hash: Option<String>,
    /// Unique identifier of the WebApp query, used to answer inline queries.
    pub query_id: Option<String>,
    /// Value mocking the `bg_color` theme parameter (hex color string).
    pub bg_color: Option<String>,
    /// Value mocking the `text_color` theme parameter (hex color string).
    pub text_color: Option<String>,
    /// Value mocking the `hint_color` theme parameter (hex color string).
    pub hint_color: Option<String>,
    /// Value mocking the `link_color` theme parameter (hex color string).
    pub link_color: Option<String>,
    /// Value mocking the `button_color` theme parameter (hex color string).
    pub button_color: Option<String>,
    /// Value mocking the `button_text_color` theme parameter (hex color
    /// string).
    pub button_text_color: Option<String>,
    /// Value mocking the `secondary_bg_color` theme parameter (hex color
    /// string).
    pub secondary_bg_color: Option<String>,
    /// Value mocking the `header_bg_color` theme parameter (hex color string).
    pub header_bg_color: Option<String>,
    /// Value mocking the `bottom_bar_bg_color` theme parameter (hex color
    /// string).
    pub bottom_bar_bg_color: Option<String>,
    /// Value mocking the `accent_text_color` theme parameter (hex color
    /// string).
    pub accent_text_color: Option<String>,
    /// Value mocking the `section_bg_color` theme parameter (hex color string).
    pub section_bg_color: Option<String>,
    /// Value mocking the `section_header_text_color` theme parameter (hex color
    /// string).
    pub section_header_text_color: Option<String>,
    /// Value mocking the `section_separator_color` theme parameter (hex color
    /// string).
    pub section_separator_color: Option<String>,
    /// Value mocking the `subtitle_text_color` theme parameter (hex color
    /// string).
    pub subtitle_text_color: Option<String>,
    /// Value mocking the `destructive_text_color` theme parameter (hex color
    /// string).
    pub destructive_text_color: Option<String>,
    /// Mocked platform identifier (e.g. `android`, `ios`, `tdesktop`).
    pub platform: Option<String>,
    /// Mocked Telegram WebApp version string (e.g. `7.0`).
    pub version: Option<String>
}

impl MockTelegramConfig {
    /// Loads configuration from a TOML file.
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_user_from_file() {
        let cfg = MockTelegramConfig::from_file("telegram-webapp.toml").expect("config");
        assert_eq!(cfg.user.unwrap().first_name, "Alice");
    }

    #[test]
    fn missing_file_is_error() {
        assert!(MockTelegramConfig::from_file("nope.toml").is_err());
    }
}
