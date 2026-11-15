// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::Deserialize;

use super::{chat::TelegramChat, user::TelegramUser};

/// Represents the complete initialization data passed to the Mini App.
/// WARNING: Always validate this data on the server using the `hash` or
/// `signature`.
#[derive(Clone, Debug, Deserialize)]
pub struct TelegramInitData {
    /// Unique identifier for the current Mini App session provided via
    /// `Telegram.WebApp.initData`.
    ///
    /// Present when the Mini App is launched from inline query results and can
    /// be used to respond through `answerWebAppQuery`.
    pub query_id: Option<String>,

    /// Unique identifier for the chat join request query that launched the Mini
    /// App.
    ///
    /// Present when the Mini App is opened from a chat join request and is
    /// required to process the query through `answerChatJoinRequestQuery`.
    pub chat_join_request_query_id: Option<String>,

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

#[cfg(test)]
mod tests {
    use serde_urlencoded::from_str;

    use super::*;

    #[test]
    fn deserialize_init_data_with_join_request_query_id() {
        let encoded = "chat_join_request_query_id=join-42&auth_date=1700000000&hash=abc";
        let data: TelegramInitData = from_str(encoded).unwrap();
        assert_eq!(data.chat_join_request_query_id.as_deref(), Some("join-42"));
        assert_eq!(data.query_id, None);
    }

    #[test]
    fn deserialize_init_data_without_join_request_query_id() {
        let encoded = "query_id=inline-7&auth_date=1700000000&hash=abc";
        let data: TelegramInitData = from_str(encoded).unwrap();
        assert_eq!(data.chat_join_request_query_id, None);
        assert_eq!(data.query_id.as_deref(), Some("inline-7"));
    }

    #[test]
    fn deserialize_init_data_missing_required_fields() {
        let encoded = "auth_date=1700000000";
        let result: Result<TelegramInitData, _> = from_str(encoded);
        assert!(result.is_err());
    }
}
