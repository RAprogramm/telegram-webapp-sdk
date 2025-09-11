use serde::{Deserialize, Serialize};

/// Result of sending a message via the Telegram Web App.
///
/// # Examples
///
/// ```rust
/// use telegram_webapp_sdk::core::types::sent_web_app_message::SentWebAppMessage;
///
/// let msg = SentWebAppMessage {
///     inline_message_id: None
/// };
/// assert!(msg.inline_message_id.is_none());
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: Option<String>
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn serialize_sent_web_app_message() {
        let msg = SentWebAppMessage {
            inline_message_id: Some("id".to_owned())
        };
        let json = to_string(&msg).unwrap();
        assert!(json.contains("id"));
        let parsed: SentWebAppMessage = from_str(&json).unwrap();
        assert_eq!(parsed.inline_message_id, msg.inline_message_id);
    }

    #[test]
    fn deserialize_sent_web_app_message_none() {
        let json = "{}";
        let parsed: SentWebAppMessage = from_str(json).unwrap();
        assert!(parsed.inline_message_id.is_none());
    }
}
