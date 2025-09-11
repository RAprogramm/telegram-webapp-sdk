use serde::{Deserialize, Serialize};

/// Indicates whether the bot can write messages to the user.
///
/// # Examples
///
/// ```rust
/// use telegram_webapp_sdk::core::types::write_access_allowed::WriteAccessAllowed;
///
/// let access = WriteAccessAllowed {
///     web_app_name: Some("my_app".to_owned())
/// };
/// assert_eq!(access.web_app_name, Some("my_app".to_owned()));
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// Name of the Web App, if the user granted access for it.
    pub web_app_name: Option<String>
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn serialize_write_access_allowed() {
        let access = WriteAccessAllowed {
            web_app_name: Some("demo".to_owned())
        };
        let json = to_string(&access).unwrap();
        assert!(json.contains("demo"));
        let parsed: WriteAccessAllowed = from_str(&json).unwrap();
        assert_eq!(parsed.web_app_name, access.web_app_name);
    }

    #[test]
    fn deserialize_write_access_allowed_none() {
        let json = "{}";
        let parsed: WriteAccessAllowed = from_str(json).unwrap();
        assert!(parsed.web_app_name.is_none());
    }
}
