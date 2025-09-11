use serde_json::to_string;
use urlencoding::encode;

use crate::mock::data::MockTelegramUser;

/// Generate a valid Telegram `initData` string from user info and params.
///
/// # Arguments
/// - `user`: Telegram user data (as `MockTelegramUser`)
/// - `auth_date`: UNIX timestamp (as string, e.g., `"1234567890"`)
/// - `hash`: mock hash string (can be `"fakehash"` or real value)
///
/// # Returns
/// Properly URL-encoded initData string for Telegram WebApp emulation.
pub fn generate_mock_init_data(user: &MockTelegramUser, auth_date: &str, hash: &str) -> String {
    let user_json = to_string(user).unwrap_or_else(|_| "{}".into());
    let encoded_user = encode(&user_json);

    format!(
        "user={}&auth_date={}&hash={}",
        encoded_user, auth_date, hash
    )
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
        let data = generate_mock_init_data(&user, auth_date, hash);

        assert!(data.contains("user="));
        assert!(data.contains("auth_date=123456"));
        assert!(data.contains("hash=hash"));
    }
}
