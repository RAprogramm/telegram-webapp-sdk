use serde::{Deserialize, Serialize};

/// Data sent to the bot when the user interacts with a Web App.
///
/// # Examples
///
/// ```rust
/// use telegram_webapp_sdk::core::types::web_app_data::WebAppData;
///
/// let data = WebAppData {
///     data:        "payload".to_owned(),
///     button_text: "Confirm".to_owned()
/// };
/// assert_eq!(data.button_text, "Confirm");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppData {
    /// Data transferred from the Web App to the bot.
    pub data:        String,
    /// Text of the button that opened the Web App.
    pub button_text: String
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, from_value, json, to_string};

    use super::*;

    #[test]
    fn serialize_web_app_data() {
        let data = WebAppData {
            data:        "test".to_owned(),
            button_text: "Send".to_owned()
        };
        let json = to_string(&data).unwrap();
        assert!(json.contains("test"));
        let parsed: WebAppData = from_str(&json).unwrap();
        assert_eq!(parsed.button_text, data.button_text);
    }

    #[test]
    fn deserialize_web_app_data_missing_field() {
        let value = json!({ "data": "test" });
        let result: Result<WebAppData, _> = from_value(value);
        assert!(result.is_err());
    }
}
