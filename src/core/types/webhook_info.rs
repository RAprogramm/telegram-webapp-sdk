use serde::{Deserialize, Serialize};

/// Contains information about the current webhook status.
///
/// # Examples
///
/// ```rust
/// use telegram_webapp_sdk::core::types::webhook_info::WebhookInfo;
///
/// let info = WebhookInfo {
///     url: "https://example.com".to_owned(),
///     has_custom_certificate: false,
///     pending_update_count: 0,
///     ip_address: None,
///     last_error_date: None,
///     last_error_message: None,
///     last_synchronization_error_date: None,
///     max_connections: None,
///     allowed_updates: None
/// };
/// assert_eq!(info.url, "https://example.com");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL.
    pub url: String,
    /// True if a self-signed certificate is used.
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery.
    pub pending_update_count: u32,
    /// Currently used IP address.
    pub ip_address: Option<String>,
    /// Unix time of the most recent delivery error.
    pub last_error_date: Option<u64>,
    /// Error message of the most recent delivery error.
    pub last_error_message: Option<String>,
    /// Unix time of the most recent synchronization error.
    pub last_synchronization_error_date: Option<u64>,
    /// Maximum allowed connections.
    pub max_connections: Option<u32>,
    /// Allowed update types for the webhook.
    pub allowed_updates: Option<Vec<String>>
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn serialize_webhook_info() {
        let info = WebhookInfo {
            url: "https://example.com".to_owned(),
            has_custom_certificate: true,
            pending_update_count: 10,
            ip_address: Some("127.0.0.1".to_owned()),
            last_error_date: Some(1),
            last_error_message: Some("error".to_owned()),
            last_synchronization_error_date: Some(2),
            max_connections: Some(40),
            allowed_updates: Some(vec!["message".to_owned()])
        };
        let json = to_string(&info).unwrap();
        assert!(json.contains("https://example.com"));
        let parsed: WebhookInfo = from_str(&json).unwrap();
        assert_eq!(parsed.url, info.url);
    }

    #[test]
    fn deserialize_webhook_info_missing_required() {
        let json = "{}";
        let result: Result<WebhookInfo, _> = from_str(json);
        assert!(result.is_err());
    }
}
