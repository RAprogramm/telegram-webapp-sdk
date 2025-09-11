use serde::Deserialize;

use super::{chat::TelegramChat, user::TelegramUser};

/// Represents the complete initialization data passed to the Mini App.
/// WARNING: Always validate this data on the server using the `hash` or
/// `signature`.
#[derive(Clone, Debug, Deserialize)]
pub struct TelegramInitData {
    /// Unique identifier for the current Mini App session.
    pub query_id: Option<String>,

    /// Information about the current Telegram user.
    pub user: Option<TelegramUser>,

    /// Information about the chat partner in private attachment menu context.
    pub receiver: Option<TelegramUser>,

    /// Information about the current chat (group, supergroup, or channel).
    pub chat: Option<TelegramChat>,

    /// Type of chat: one of "private", "group", "supergroup", "channel", or
    /// "sender".
    pub chat_type: Option<String>,

    /// Globally unique chat instance identifier.
    pub chat_instance: Option<String>,

    /// Value of the `start_param` or `startattach` passed in the launch URL.
    pub start_param: Option<String>,

    /// Time (in seconds) after which the Mini App may send a message via
    /// `answerWebAppQuery`.
    pub can_send_after: Option<u64>,

    /// Unix timestamp of when the init data was generated.
    pub auth_date: u64,

    /// HMAC-SHA256 hash used to verify data integrity on the server.
    pub hash: String,

    /// Ed25519 signature used for third-party data validation (optional).
    pub signature: Option<String>
}
