use std::{
    fs,
    io::{Error, ErrorKind}
};

use serde::Deserialize;

use super::data::MockTelegramUser;

/// Configuration for mocking Telegram environment.
#[derive(Default, Deserialize)]
pub struct MockTelegramConfig {
    pub user: Option<MockTelegramUser>,
    pub auth_date: Option<String>,
    pub hash: Option<String>,
    pub bg_color: Option<String>,
    pub text_color: Option<String>,
    pub hint_color: Option<String>,
    pub link_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>,
    pub secondary_bg_color: Option<String>,
    pub header_bg_color: Option<String>,
    pub bottom_bar_bg_color: Option<String>,
    pub accent_text_color: Option<String>,
    pub section_bg_color: Option<String>,
    pub section_header_text_color: Option<String>,
    pub section_separator_color: Option<String>,
    pub subtitle_text_color: Option<String>,
    pub destructive_text_color: Option<String>,
    pub platform: Option<String>,
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
