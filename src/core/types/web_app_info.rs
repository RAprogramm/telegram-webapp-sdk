use serde::{Deserialize, Serialize};

/// Describes a Web App.
///
/// # Examples
///
/// ```rust
/// use telegram_webapp_sdk::core::types::web_app_info::WebAppInfo;
///
/// let info = WebAppInfo {
///     url: "https://example.com".to_owned()
/// };
/// assert_eq!(info.url, "https://example.com");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// HTTPS URL of a Web App to open.
    pub url: String
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, from_value, json, to_string};

    use super::*;

    #[test]
    fn serialize_web_app_info() {
        let info = WebAppInfo {
            url: "https://t.me".to_owned()
        };
        let json = to_string(&info).unwrap();
        assert_eq!(json, "{\"url\":\"https://t.me\"}");
        let value: WebAppInfo = from_str(&json).unwrap();
        assert_eq!(value.url, info.url);
    }

    #[test]
    fn deserialize_web_app_info_missing_url() {
        let data = json!({});
        let result: Result<WebAppInfo, _> = from_value(data);
        assert!(result.is_err());
    }
}
