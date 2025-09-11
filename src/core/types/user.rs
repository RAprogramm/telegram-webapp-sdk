use serde::{Deserialize, Serialize};

/// Represents a Telegram user in the context of a Mini App.
///
/// # Examples
///
/// ```rust
/// use serde_json::{from_str, to_string};
/// use telegram_webapp_sdk::core::types::user::TelegramUser;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let user = TelegramUser {
///     id: 1,
///     is_bot: Some(false),
///     first_name: "Alice".into(),
///     last_name: Some("Smith".into()),
///     username: Some("alice".into()),
///     language_code: Some("en".into()),
///     is_premium: Some(true),
///     added_to_attachment_menu: Some(false),
///     allows_write_to_pm: Some(true),
///     photo_url: Some("https://example.com/photo.jpg".into())
/// };
/// let json = to_string(&user)?;
/// let parsed: TelegramUser = from_str(&json)?;
/// assert_eq!(parsed.id, user.id);
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelegramUser {
    /// Unique Telegram user or bot ID (64-bit unsigned integer).
    pub id: u64,

    /// Whether the user is a bot (only present for `receiver` field).
    pub is_bot: Option<bool>,

    /// User's first name.
    pub first_name: String,

    /// User's last name (optional).
    pub last_name: Option<String>,

    /// Telegram username (optional).
    pub username: Option<String>,

    /// IETF language code (e.g., "en", "ru").
    pub language_code: Option<String>,

    /// Whether the user is a Telegram Premium subscriber.
    pub is_premium: Option<bool>,

    /// True if the user added the bot to their attachment menu.
    pub added_to_attachment_menu: Option<bool>,

    /// True if the user allowed the bot to message them.
    pub allows_write_to_pm: Option<bool>,

    /// Profile photo URL (JPEG or SVG), if available.
    pub photo_url: Option<String>
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn serialize_user() {
        let user = TelegramUser {
            id: 42,
            is_bot: Some(false),
            first_name: "Bob".into(),
            last_name: None,
            username: Some("bob".into()),
            language_code: Some("en".into()),
            is_premium: Some(false),
            added_to_attachment_menu: Some(false),
            allows_write_to_pm: Some(true),
            photo_url: Some("https://example.com/avatar.jpg".into())
        };
        let json = to_string(&user).unwrap();
        assert!(json.contains("Bob"));
        let parsed: TelegramUser = from_str(&json).unwrap();
        assert_eq!(parsed.id, user.id);
    }

    #[test]
    fn deserialize_user_missing_required() {
        let json = r#"{"first_name":"John"}"#; // missing `id`
        let res: Result<TelegramUser, _> = from_str(json);
        assert!(res.is_err());
    }
}
