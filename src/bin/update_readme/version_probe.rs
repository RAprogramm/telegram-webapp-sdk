use std::time::Duration;

use regex::Regex;
use reqwest::{blocking::Client, header::ACCEPT};
use thiserror::Error;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const VERSION_PATTERN: &str = r#"parameters->version_\s*=\s*\"(?P<version>\d+(?:\.\d+)*)\""#;

#[derive(Debug, Error)]
pub enum VersionDiscoveryError {
    #[error("latest version probe url is empty")]
    EmptyUrl,
    #[error("failed to build HTTP client: {0}")]
    ClientBuild(reqwest::Error),
    #[error("failed to fetch {url}: {source}")]
    Request {
        url:    String,
        source: reqwest::Error
    },
    #[error("failed to read body from {url}: {source}")]
    BodyRead {
        url:    String,
        source: reqwest::Error
    },
    #[error("failed to compile latest version pattern: {0}")]
    Pattern(regex::Error),
    #[error("latest version marker not found in {url}")]
    VersionNotFound { url: String }
}

pub fn discover_latest_version(probe_url: &str) -> Result<String, VersionDiscoveryError> {
    if probe_url.trim().is_empty() {
        return Err(VersionDiscoveryError::EmptyUrl);
    }

    let client = Client::builder()
        .user_agent(format!(
            "{}/{} (+https://github.com/RAprogramm/telegram-webapp-sdk)",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(VersionDiscoveryError::ClientBuild)?;

    let response = client
        .get(probe_url)
        .header(ACCEPT, "text/plain, text/x-c++src, text/html")
        .send()
        .map_err(|source| VersionDiscoveryError::Request {
            url: probe_url.to_owned(),
            source
        })?
        .error_for_status()
        .map_err(|source| VersionDiscoveryError::Request {
            url: probe_url.to_owned(),
            source
        })?;

    let body = response
        .text()
        .map_err(|source| VersionDiscoveryError::BodyRead {
            url: probe_url.to_owned(),
            source
        })?;

    extract_version(probe_url, &body)
}

fn extract_version(probe_url: &str, body: &str) -> Result<String, VersionDiscoveryError> {
    let regex = Regex::new(VERSION_PATTERN).map_err(VersionDiscoveryError::Pattern)?;
    let captures = regex
        .captures(body)
        .ok_or_else(|| VersionDiscoveryError::VersionNotFound {
            url: probe_url.to_owned()
        })?;
    let version = captures
        .name("version")
        .ok_or_else(|| VersionDiscoveryError::VersionNotFound {
            url: probe_url.to_owned()
        })?
        .as_str()
        .to_owned();

    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_version_reads_numeric_segments() {
        let input = "// parameters->version_ = \"8.2\";";
        let result = extract_version("https://example.com", input).expect("version");
        assert_eq!(result, "8.2");
    }

    #[test]
    fn extract_version_supports_multiple_segments() {
        let input = "parameters->version_ = \"7.10\";";
        let result = extract_version("https://example.com", input).expect("version");
        assert_eq!(result, "7.10");
    }

    #[test]
    fn extract_version_reports_missing_marker() {
        let error = extract_version("https://example.com", "int version = 1;")
            .expect_err("missing version");
        match error {
            VersionDiscoveryError::VersionNotFound {
                url
            } => {
                assert_eq!(url, "https://example.com");
            }
            other => panic!("unexpected error: {other:?}")
        }
    }
}
