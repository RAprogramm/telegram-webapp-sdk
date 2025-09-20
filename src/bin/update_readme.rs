use std::{env, fs, path::PathBuf};

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use thiserror::Error;

const BADGES_START: &str = "<!-- webapp_api_badges:start -->";
const BADGES_END: &str = "<!-- webapp_api_badges:end -->";
const SUMMARY_START: &str = "<!-- webapp_api_summary:start -->";
const SUMMARY_END: &str = "<!-- webapp_api_summary:end -->";
const DEFAULT_SOURCE_URL: &str = "https://core.telegram.org/bots/webapps";
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
    #[error("failed to read file {path}: {source}")]
    ReadFile {
        path:   String,
        source: std::io::Error
    },
    #[error("commit {commit} declared in metadata not found in WEBAPP_API.md")]
    CommitNotReferenced { commit: String },
    #[error("README.md marker {marker} not found")]
    MarkerMissing { marker: String },
    #[error("failed to parse repository url from Cargo.toml: {0}")]
    RepositoryParse(toml::de::Error),
    #[error("repository field missing in Cargo.toml")]
    RepositoryMissing,
    #[error("failed to write README.md: {0}")]
    WriteReadme(std::io::Error)
}

#[derive(Debug, Deserialize)]
struct WebAppApiStatusTable {
    #[serde(rename = "webapp_api_status")]
    status: WebAppApiStatusRaw
}

#[derive(Debug, Deserialize)]
struct WebAppApiStatusRaw {
    latest_version:      String,
    covered_version:     String,
    coverage_commit:     String,
    #[serde(default)]
    coverage_date:       Option<String>,
    #[serde(default)]
    source_url:          Option<String>,
    #[serde(default)]
    coverage_commit_url: Option<String>
}

#[derive(Debug)]
struct WebAppApiStatus {
    latest_version:      String,
    covered_version:     String,
    coverage_commit:     String,
    coverage_date:       Option<String>,
    source_url:          String,
    coverage_commit_url: Option<String>
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    repository: Option<String>
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
        fs::read_to_string(&webapp_api_path).map_err(|source| ReadmeUpdateError::ReadFile {
            path: webapp_api_path.display().to_string(),
            source
        })?;
    let readme_content =
        fs::read_to_string(&readme_path).map_err(|source| ReadmeUpdateError::ReadFile {
            path: readme_path.display().to_string(),
            source
        })?;
    let cargo_toml_content =
        fs::read_to_string(&cargo_toml_path).map_err(|source| ReadmeUpdateError::ReadFile {
            path: cargo_toml_path.display().to_string(),
            source
        })?;

    let status = parse_status(&webapp_api_content)?;
    let repository = parse_repository(&cargo_toml_content)?;
    let commit_url = status.coverage_commit_url.clone().unwrap_or_else(|| {
        format!(
            "{}/commit/{}",
            repository.trim_end_matches('/'),
            status.coverage_commit
        )
    });

    let badges_block = render_badges(&status, &commit_url);
    let summary_block = render_summary(&status, &commit_url);

    let with_badges = replace_section(&readme_content, BADGES_START, BADGES_END, &badges_block)?;
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
                    latest_version:      status_raw.latest_version,
                    covered_version:     status_raw.covered_version,
                    coverage_commit:     status_raw.coverage_commit,
                    coverage_date:       status_raw.coverage_date,
                    source_url:          status_raw
                        .source_url
                        .unwrap_or_else(|| DEFAULT_SOURCE_URL.to_owned()),
                    coverage_commit_url: status_raw.coverage_commit_url
                });
            }
            search = &after_start[end_offset + 3..];
        } else {
            break;
        }
    }

    Err(ReadmeUpdateError::MetadataCommentMissing)
}

fn parse_repository(cargo_toml: &str) -> Result<String, ReadmeUpdateError> {
    let parsed: CargoToml =
        toml::from_str(cargo_toml).map_err(ReadmeUpdateError::RepositoryParse)?;
    parsed
        .package
        .repository
        .ok_or(ReadmeUpdateError::RepositoryMissing)
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
        "[![{alt_label}](https://img.shields.io/badge/{badge_label}-{latest_encoded}-blue)]({source})\n[![Coverage](https://img.shields.io/badge/{coverage_label}-{coverage_message}-{coverage_colour})]({commit_url})\n",
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
        "**WebApp API coverage:** version `{covered}` {relation}. Synced in commit [{commit_short}]({commit_url}){date_suffix}.\n",
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
        let markdown = "<!--\n[webapp_api_status]\nlatest_version = \"7.10\"\ncovered_version = \"7.10\"\ncoverage_commit = \"7a2555c\"\ncoverage_date = \"2025-09-11\"\nsource_url = \"https://example.com\"\n-->\nother content 7a2555c";
        let status = parse_status(markdown).expect("status");
        assert_eq!(status.latest_version, "7.10");
        assert_eq!(status.covered_version, "7.10");
        assert_eq!(status.coverage_commit, "7a2555c");
        assert_eq!(status.coverage_date.as_deref(), Some("2025-09-11"));
        assert_eq!(status.source_url, "https://example.com");
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
            "start<!-- webapp_api_badges:start -->\nline\n<!-- webapp_api_badges:end -->end"
        );
    }

    #[test]
    fn render_badges_encodes_values() {
        let status = WebAppApiStatus {
            latest_version:      "7.10".to_owned(),
            covered_version:     "7.10".to_owned(),
            coverage_commit:     "abcdef123456".to_owned(),
            coverage_date:       None,
            source_url:          "https://example.com".to_owned(),
            coverage_commit_url: None
        };
        let badges = render_badges(&status, "https://repo/commit/abcdef1");
        assert!(badges.contains("abcdef1"));
        assert!(badges.contains("7.10"));
    }
}
