use serde::Deserialize;

/// Represents a Telegram user in the context of a Mini App.
#[derive(Deserialize, Debug)]
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
