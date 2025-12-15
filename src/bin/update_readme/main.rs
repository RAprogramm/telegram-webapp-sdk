// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![allow(
    non_shorthand_field_patterns,
    reason = "derive-generated source access needs renames"
)]

mod version_probe;

use std::{env, fs, path::PathBuf};

use masterror::Error;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use version_probe::{VersionDiscoveryError, discover_latest_version};

const BADGES_START: &str = "<!-- webapp_api_badges:start -->";
const BADGES_END: &str = "<!-- webapp_api_badges:end -->";
const SUMMARY_START: &str = "<!-- webapp_api_summary:start -->";
const SUMMARY_END: &str = "<!-- webapp_api_summary:end -->";
const MSRV_START: &str = "<!-- msrv_badge:start -->";
const MSRV_END: &str = "<!-- msrv_badge:end -->";
const DEFAULT_SOURCE_URL: &str = "https://core.telegram.org/bots/webapps";
const DEFAULT_VERSION_PROBE_URL: &str = "https://raw.githubusercontent.com/tdlib/telegram-bot-api/master/telegram-bot-api/telegram-bot-api.cpp";
const BADGE_LINK_LABEL: &str = "Telegram WebApp API";

const BADGE_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'_').remove(b'.');

#[derive(Debug, Error)]
enum ReadmeUpdateError {
    #[error("WEBAPP_API.md comment with [webapp_api_status] not found")]
    MetadataCommentMissing,
    #[error("failed to parse webapp_api_status comment: {0}")]
    MetadataParse(toml::de::Error),
    #[error("environment variable CARGO_MANIFEST_DIR not set: {0}")]
    ManifestDir(env::VarError),
    #[error("failed to read file {path}: {error}")]
    ReadFile {
        path:  String,
        #[source]
        error: std::io::Error
    },
    #[error("commit {commit} declared in metadata not found in WEBAPP_API.md")]
    CommitNotReferenced { commit: String },
    #[error("README.md marker {marker} not found")]
    MarkerMissing { marker: String },
    #[error("failed to parse repository url from Cargo.toml: {0}")]
    RepositoryParse(toml::de::Error),
    #[error("repository field missing in Cargo.toml")]
    RepositoryMissing,
    #[error("rust-version field missing in Cargo.toml")]
    RustVersionMissing,
    #[error("failed to write README.md: {0}")]
    WriteReadme(std::io::Error),
    #[error("failed to determine latest WebApp API version: {0}")]
    LatestVersion(VersionDiscoveryError)
}

#[derive(Debug, Deserialize)]
struct WebAppApiStatusTable {
    #[serde(rename = "webapp_api_status")]
    status: WebAppApiStatusRaw
}

#[derive(Debug, Deserialize)]
struct WebAppApiStatusRaw {
    latest_version:           String,
    covered_version:          String,
    coverage_commit:          String,
    #[serde(default)]
    coverage_date:            Option<String>,
    #[serde(default)]
    source_url:               Option<String>,
    #[serde(default)]
    coverage_commit_url:      Option<String>,
    #[serde(default)]
    latest_version_probe_url: Option<String>
}

#[derive(Debug)]
struct WebAppApiStatus {
    latest_version:           String,
    covered_version:          String,
    coverage_commit:          String,
    coverage_date:            Option<String>,
    source_url:               String,
    coverage_commit_url:      Option<String>,
    latest_version_probe_url: String
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    repository:   Option<String>,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: CargoPackage
}

fn main() -> Result<(), ReadmeUpdateError> {
    run()
}

fn run() -> Result<(), ReadmeUpdateError> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(ReadmeUpdateError::ManifestDir)?;
    let root = PathBuf::from(manifest_dir);

    let webapp_api_path = root.join("WEBAPP_API.md");
    let readme_path = root.join("README.md");
    let cargo_toml_path = root.join("Cargo.toml");

    let webapp_api_content =
        fs::read_to_string(&webapp_api_path).map_err(|error| ReadmeUpdateError::ReadFile {
            path: webapp_api_path.display().to_string(),
            error
        })?;
    let readme_content =
        fs::read_to_string(&readme_path).map_err(|error| ReadmeUpdateError::ReadFile {
            path: readme_path.display().to_string(),
            error
        })?;
    let cargo_toml_content =
        fs::read_to_string(&cargo_toml_path).map_err(|error| ReadmeUpdateError::ReadFile {
            path: cargo_toml_path.display().to_string(),
            error
        })?;

    let mut status = parse_status(&webapp_api_content)?;
    let latest_source_version = discover_latest_version(status.latest_version_probe_url.as_str())
        .map_err(ReadmeUpdateError::LatestVersion)?;
    if status.latest_version != latest_source_version {
        eprintln!(
            "WEBAPP_API.md declares latest version {} but source reports {}. Using source version.",
            status.latest_version, latest_source_version
        );
    }
    status.latest_version = latest_source_version;
    let cargo = parse_cargo_toml(&cargo_toml_content)?;
    let repository = cargo
        .package
        .repository
        .ok_or(ReadmeUpdateError::RepositoryMissing)?;
    let rust_version = cargo
        .package
        .rust_version
        .ok_or(ReadmeUpdateError::RustVersionMissing)?;
    let commit_url = status.coverage_commit_url.clone().unwrap_or_else(|| {
        format!(
            "{}/commit/{}",
            repository.trim_end_matches('/'),
            status.coverage_commit
        )
    });

    let badges_block = render_badges(&status, &commit_url);
    let summary_block = render_summary(&status, &commit_url);
    let msrv_block = render_msrv_badge(&rust_version);

    let with_msrv = replace_section(&readme_content, MSRV_START, MSRV_END, &msrv_block)?;
    let with_badges = replace_section(&with_msrv, BADGES_START, BADGES_END, &badges_block)?;
    let updated = replace_section(&with_badges, SUMMARY_START, SUMMARY_END, &summary_block)?;

    if updated != readme_content {
        fs::write(&readme_path, updated).map_err(ReadmeUpdateError::WriteReadme)?;
    }

    Ok(())
}

fn parse_status(content: &str) -> Result<WebAppApiStatus, ReadmeUpdateError> {
    let mut search = content;
    while let Some(start) = search.find("<!--") {
        let after_start = &search[start + 4..];
        if let Some(end_offset) = after_start.find("-->") {
            let comment = &after_start[..end_offset];
            if comment.contains("[webapp_api_status]") {
                let trimmed = comment.trim();
                let table: WebAppApiStatusTable =
                    toml::from_str(trimmed).map_err(ReadmeUpdateError::MetadataParse)?;
                let status_raw = table.status;
                if !content.contains(&status_raw.coverage_commit) {
                    return Err(ReadmeUpdateError::CommitNotReferenced {
                        commit: status_raw.coverage_commit
                    });
                }
                return Ok(WebAppApiStatus {
                    latest_version:           status_raw.latest_version,
                    covered_version:          status_raw.covered_version,
                    coverage_commit:          status_raw.coverage_commit,
                    coverage_date:            status_raw.coverage_date,
                    source_url:               status_raw
                        .source_url
                        .unwrap_or_else(|| DEFAULT_SOURCE_URL.to_owned()),
                    coverage_commit_url:      status_raw.coverage_commit_url,
                    latest_version_probe_url: status_raw
                        .latest_version_probe_url
                        .unwrap_or_else(|| DEFAULT_VERSION_PROBE_URL.to_owned())
                });
            }
            search = &after_start[end_offset + 3..];
        } else {
            break;
        }
    }

    Err(ReadmeUpdateError::MetadataCommentMissing)
}

fn parse_cargo_toml(cargo_toml: &str) -> Result<CargoToml, ReadmeUpdateError> {
    toml::from_str(cargo_toml).map_err(ReadmeUpdateError::RepositoryParse)
}

fn render_badges(status: &WebAppApiStatus, commit_url: &str) -> String {
    let latest_encoded = encode_badge_component(&status.latest_version);
    let badge_label = encode_badge_component(BADGE_LINK_LABEL);
    let coverage_label = encode_badge_component("Coverage");
    let commit_short: String = status.coverage_commit.chars().take(7).collect();
    let is_up_to_date = status.covered_version == status.latest_version;
    let status_text = if is_up_to_date {
        format!("up to date ({})", commit_short)
    } else {
        format!("update needed ({})", commit_short)
    };
    let coverage_message = encode_badge_component(&status_text);
    let coverage_colour = if is_up_to_date {
        "brightgreen"
    } else {
        "orange"
    };

    format!(
        "[![{alt_label}](https://img.shields.io/badge/{badge_label}-{latest_encoded}-blue)]({source})
[![Coverage](https://img.shields.io/badge/{coverage_label}-{coverage_message}-{coverage_colour})]({commit_url})
",
        alt_label = BADGE_LINK_LABEL,
        badge_label = badge_label,
        latest_encoded = latest_encoded,
        source = status.source_url,
        coverage_label = coverage_label,
        coverage_message = coverage_message,
        coverage_colour = coverage_colour,
        commit_url = commit_url
    )
}

fn render_msrv_badge(rust_version: &str) -> String {
    let version_encoded = encode_badge_component(rust_version);
    format!("![MSRV](https://img.shields.io/badge/MSRV-{version_encoded}-blue)\n")
}

fn render_summary(status: &WebAppApiStatus, commit_url: &str) -> String {
    let commit_short: String = status.coverage_commit.chars().take(7).collect();
    let relation = if status.covered_version == status.latest_version {
        format!(
            "matches the latest Telegram WebApp API release `{}`",
            status.latest_version
        )
    } else {
        format!(
            "lags behind the latest Telegram WebApp API release `{}`",
            status.latest_version
        )
    };
    let date_suffix = status
        .coverage_date
        .as_deref()
        .map(|date| format!(" (recorded on {date})"))
        .unwrap_or_default();

    format!(
        "**WebApp API coverage:** version `{covered}` {relation}. Synced in commit [{commit_short}]({commit_url}){date_suffix}.
",
        covered = status.covered_version,
        relation = relation,
        commit_short = commit_short,
        commit_url = commit_url,
        date_suffix = date_suffix
    )
}

fn replace_section(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    replacement: &str
) -> Result<String, ReadmeUpdateError> {
    let start = content
        .find(start_marker)
        .ok_or_else(|| ReadmeUpdateError::MarkerMissing {
            marker: start_marker.to_owned()
        })?;
    let after_start = start + start_marker.len();
    let tail = &content[after_start..];
    let end_offset = tail
        .find(end_marker)
        .ok_or_else(|| ReadmeUpdateError::MarkerMissing {
            marker: end_marker.to_owned()
        })?;
    let end = after_start + end_offset;

    let mut output = String::with_capacity(content.len() + replacement.len());
    output.push_str(&content[..after_start]);
    output.push('\n');
    let trimmed = replacement.trim_matches('\n');
    if !trimmed.is_empty() {
        output.push_str(trimmed);
        output.push('\n');
    }
    output.push_str(&content[end..]);
    Ok(output)
}

fn encode_badge_component(value: &str) -> String {
    utf8_percent_encode(value, BADGE_ENCODE_SET).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_extracts_metadata() {
        let markdown = "<!--
[webapp_api_status]
latest_version = \"7.10\"
covered_version = \"7.10\"
coverage_commit = \"7a2555c\"
coverage_date = \"2025-09-11\"
source_url = \"https://example.com\"
-->
other content 7a2555c";
        let status = parse_status(markdown).expect("status");
        assert_eq!(status.latest_version, "7.10");
        assert_eq!(status.covered_version, "7.10");
        assert_eq!(status.coverage_commit, "7a2555c");
        assert_eq!(status.coverage_date.as_deref(), Some("2025-09-11"));
        assert_eq!(status.source_url, "https://example.com");
        assert_eq!(status.latest_version_probe_url, DEFAULT_VERSION_PROBE_URL);
    }

    #[test]
    fn parse_status_reads_custom_probe_url() {
        let markdown = "<!--
[webapp_api_status]
latest_version = \"7.10\"
covered_version = \"7.10\"
coverage_commit = \"7a2555c\"
latest_version_probe_url = \"https://example.com/version.txt\"
-->
7a2555c";
        let status = parse_status(markdown).expect("status");
        assert_eq!(
            status.latest_version_probe_url,
            "https://example.com/version.txt"
        );
    }

    #[test]
    fn replace_section_substitutes_between_markers() {
        let original = "start<!-- webapp_api_badges:start --><!-- webapp_api_badges:end -->end";
        let updated = replace_section(
            original,
            "<!-- webapp_api_badges:start -->",
            "<!-- webapp_api_badges:end -->",
            "line"
        )
        .expect("replace");
        assert_eq!(
            updated,
            "start<!-- webapp_api_badges:start -->
line
<!-- webapp_api_badges:end -->end"
        );
    }

    #[test]
    fn render_badges_encodes_values() {
        let status = WebAppApiStatus {
            latest_version:           "7.10".to_owned(),
            covered_version:          "7.10".to_owned(),
            coverage_commit:          "abcdef123456".to_owned(),
            coverage_date:            None,
            source_url:               "https://example.com".to_owned(),
            coverage_commit_url:      None,
            latest_version_probe_url: DEFAULT_VERSION_PROBE_URL.to_owned()
        };
        let badges = render_badges(&status, "https://repo/commit/abcdef1");
        assert!(badges.contains("abcdef1"));
        assert!(badges.contains("7.10"));
    }

    #[test]
    fn parse_cargo_toml_extracts_fields() {
        let toml = r#"
[package]
name = "test"
version = "1.0.0"
rust-version = "1.91"
repository = "https://github.com/test/test"
"#;
        let cargo = parse_cargo_toml(toml).expect("parse");
        assert_eq!(cargo.package.rust_version.as_deref(), Some("1.91"));
        assert_eq!(
            cargo.package.repository.as_deref(),
            Some("https://github.com/test/test")
        );
    }

    #[test]
    fn parse_cargo_toml_handles_missing_optional_fields() {
        let toml = r#"
[package]
name = "test"
version = "1.0.0"
"#;
        let cargo = parse_cargo_toml(toml).expect("parse");
        assert!(cargo.package.rust_version.is_none());
        assert!(cargo.package.repository.is_none());
    }

    #[test]
    fn render_msrv_badge_formats_correctly() {
        let badge = render_msrv_badge("1.91");
        assert!(badge.contains("MSRV"));
        assert!(badge.contains("1.91"));
        assert!(badge.contains("img.shields.io"));
    }

    #[test]
    fn render_msrv_badge_encodes_special_chars() {
        let badge = render_msrv_badge("1.91.0");
        assert!(badge.contains("1.91.0"));
    }
}
