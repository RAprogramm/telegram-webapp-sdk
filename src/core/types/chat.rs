use serde::Deserialize;

/// Represents a chat context (group, supergroup, or channel).
#[derive(Clone, Debug, Deserialize)]
pub struct TelegramChat {
    /// Unique identifier of the chat.
    pub id: u64,

    /// Chat type. One of: "group", "supergroup", or "channel".
    #[serde(rename = "type")]
    pub kind: String,

    /// Title of the chat.
    pub title: String,

    /// Public username of the chat (if available).
    pub username: Option<String>,

    /// Chat photo URL (JPEG or SVG), if available.
    pub photo_url: Option<String>
}
