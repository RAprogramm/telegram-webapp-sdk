// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde_json::to_string;
use urlencoding::encode;

use crate::mock::data::MockTelegramUser;

/// Generate a valid Telegram `initData` string from user info and params.
///
/// # Arguments
/// - `user`: Telegram user data (as `MockTelegramUser`)
/// - `auth_date`: UNIX timestamp (as string, e.g., `"1234567890"`)
/// - `hash`: mock hash string (can be `"fakehash"` or real value)
/// - `query_id`: optional inline query identifier to embed in the payload
///
/// # Returns
/// Properly URL-encoded initData string for Telegram WebApp emulation.
pub fn generate_mock_init_data(
    user: &MockTelegramUser,
    auth_date: &str,
    hash: &str,
    query_id: Option<&str>
) -> String {
    let user_json = to_string(user).unwrap_or_else(|_| "{}".into());
    let encoded_user = encode(&user_json);
    let mut init_data = String::with_capacity(64);

    if let Some(id) = query_id {
        init_data.push_str("query_id=");
        init_data.push_str(encode(id).as_ref());
        init_data.push('&');
    }

    init_data.push_str("user=");
    init_data.push_str(encoded_user.as_ref());
    init_data.push('&');
    init_data.push_str("auth_date=");
    init_data.push_str(auth_date);
    init_data.push('&');
    init_data.push_str("hash=");
    init_data.push_str(hash);

    init_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_expected_init_data() {
        let user = MockTelegramUser {
            id: 1,
            first_name: "Dev".into(),
            ..Default::default()
        };
        let auth_date = "123456";
        let hash = "hash";
        let data = generate_mock_init_data(&user, auth_date, hash, None);

        assert!(data.contains("user="));
        assert!(data.contains("auth_date=123456"));
        assert!(data.contains("hash=hash"));
        assert!(!data.starts_with("query_id="));
    }

    #[test]
    fn adds_query_id_when_present() {
        let user = MockTelegramUser {
            id: 1,
            first_name: "Dev".into(),
            ..Default::default()
        };
        let data = generate_mock_init_data(&user, "123456", "hash", Some("inline:42/ä"));

        assert!(data.starts_with("query_id=inline%3A42%2F%C3%A4&"));
        assert!(data.contains("hash=hash"));
    }
}
