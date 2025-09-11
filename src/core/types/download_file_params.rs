use serde::{Deserialize, Serialize};

/// Parameters for
/// [`TelegramWebApp::download_file`](crate::webapp::TelegramWebApp::download_file).
///
///
/// This structure mirrors the object expected by the `downloadFile` method in
/// the Telegram Web App JavaScript API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadFileParams<'a> {
    /// Remote URL of the file to download.
    pub url: &'a str,

    /// Optional name for the downloaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<&'a str>,

    /// Optional MIME type of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<&'a str>
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn serialize_download_file_params() {
        let params = DownloadFileParams {
            url:       "https://example.com/data.bin",
            file_name: Some("data.bin"),
            mime_type: Some("application/octet-stream")
        };
        let json = to_string(&params).expect("serialize");
        let parsed: DownloadFileParams = from_str(&json).expect("deserialize");
        assert_eq!(parsed.url, params.url);
        assert_eq!(parsed.file_name, params.file_name);
        assert_eq!(parsed.mime_type, params.mime_type);
    }
}
