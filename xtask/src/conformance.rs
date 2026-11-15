// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! `WebApp` API conformance check: every entry of `WEBAPP_API.md` must exist
//! in the official `telegram-web-app.js` bridge script.
//!
//! The checklist is the public coverage claim of the SDK. When Telegram
//! renames or drops a bridge member, the checklist silently rots; this task
//! fails (locally and in CI) as soon as a listed name disappears from the
//! bridge, pointing at exactly which entries drifted.

use std::{collections::HashSet, fs, path::Path, time::Duration};

use regex::Regex;
use reqwest::blocking::Client;

use crate::{TaskError, io_error, repo_root};

const BRIDGE_URL: &str = "https://telegram.org/js/telegram-web-app.js";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// Runs the conformance check.
///
/// `js_file` selects a local copy of the bridge script; without it the
/// script is downloaded (falling back to the cache under `target/` when the
/// network is unreachable).
///
/// # Errors
/// Returns [`TaskError`] when the checklist or bridge cannot be read, or when
/// checklist entries are missing from the bridge.
pub fn run(js_file: Option<&Path>) -> Result<(), TaskError> {
    let root = repo_root()?;
    let checklist = read_to_string(&root.join("WEBAPP_API.md"))?;
    let bridge = match js_file {
        Some(path) => read_to_string(path)?,
        None => fetch_bridge(&root)?
    };

    let items = checklist_items(&checklist)?;
    let names = bridge_names(&bridge)?;
    let mut missing: Vec<&str> = items
        .iter()
        .map(String::as_str)
        .filter(|name| !names.contains(*name))
        .collect();
    missing.sort_unstable();

    if missing.is_empty() {
        println!(
            "conformance: all {} WEBAPP_API.md entries exist in the official bridge",
            items.len()
        );
        return Ok(());
    }
    Err(TaskError::ConformanceDrift {
        missing: missing.join(", ")
    })
}

/// Collects every API name claimed by the checklist: `- [x] name` entries
/// (skipping the `WebAppInitData` section, whose fields the bridge parses
/// generically and never names) and the event names of the background-event
/// table.
fn checklist_items(markdown: &str) -> Result<Vec<String>, TaskError> {
    let entry = build_regex(r"^- \[[x ]\] (?P<name>[A-Za-z_][A-Za-z0-9_]*)")?;
    let event = build_regex(r"^\| `(?P<name>[A-Za-z_][A-Za-z0-9_]*)` \|")?;
    let mut items = Vec::new();
    let mut in_init_data_section = false;
    for line in markdown.lines() {
        if line.starts_with('#') {
            in_init_data_section = line.contains("WebAppInitData");
            continue;
        }
        if in_init_data_section {
            continue;
        }
        if let Some(captures) = entry.captures(line).or_else(|| event.captures(line))
            && let Some(name) = captures.name("name")
        {
            items.push(name.as_str().to_string());
        }
    }
    Ok(items)
}

/// Collects every identifier the bridge script exposes: assigned properties
/// (`WebApp.ready =`, `bottomButton.show =`), `defineProperty` names,
/// quoted event and field names, function declarations, and object-literal
/// methods.
fn bridge_names(script: &str) -> Result<HashSet<String>, TaskError> {
    let patterns = [
        r"\.([A-Za-z_$][A-Za-z0-9_$]*)\s*=",
        r"defineProperty\([^,]+,\s*'([A-Za-z_$][A-Za-z0-9_$]*)'",
        r"'([A-Za-z_$][A-Za-z0-9_$]*)'",
        r#""([A-Za-z_$][A-Za-z0-9_$]*)""#,
        r"\bfunction\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\(",
        r"(?m)(?:^\s*|[{,]\s*)([A-Za-z_$][A-Za-z0-9_$]*)\s*:\s*function"
    ];
    let mut names = HashSet::new();
    for pattern in patterns {
        let regex = build_regex(pattern)?;
        for captures in regex.captures_iter(script) {
            if let Some(name) = captures.get(1) {
                names.insert(name.as_str().to_string());
            }
        }
    }
    Ok(names)
}

/// Downloads the bridge script, caching it under `target/conformance/` and
/// falling back to that cache when the network is unreachable.
fn fetch_bridge(root: &Path) -> Result<String, TaskError> {
    let cache = root
        .join("target")
        .join("conformance")
        .join("telegram-web-app.js");
    let client = Client::builder()
        .user_agent(format!(
            "{}/{} (+https://github.com/RAprogramm/telegram-webapp-sdk)",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|error| TaskError::ConformanceFetch {
            url: BRIDGE_URL.to_string(),
            error
        })?;
    match client
        .get(BRIDGE_URL)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
    {
        Ok(response) => match response.text() {
            Ok(body) => {
                write_cached_bridge(&cache, &body)?;
                Ok(body)
            }
            Err(error) => Err(TaskError::ConformanceFetch {
                url: BRIDGE_URL.to_string(),
                error
            })
        },
        Err(error) => match fs::read_to_string(&cache) {
            Ok(cached) => {
                eprintln!(
                    "conformance: could not fetch {BRIDGE_URL} ({error}); using cached copy at {}",
                    cache.display()
                );
                Ok(cached)
            }
            Err(_) => Err(TaskError::ConformanceFetch {
                url: BRIDGE_URL.to_string(),
                error
            })
        }
    }
}

fn write_cached_bridge(cache: &Path, body: &str) -> Result<(), TaskError> {
    if let Some(parent) = cache.parent() {
        fs::create_dir_all(parent).map_err(|error| io_error("create", parent, error))?;
    }
    fs::write(cache, body).map_err(|error| io_error("write", cache, error))
}

fn read_to_string(path: &Path) -> Result<String, TaskError> {
    fs::read_to_string(path).map_err(|error| io_error("read", path, error))
}

fn build_regex(pattern: &str) -> Result<Regex, TaskError> {
    Regex::new(pattern).map_err(|error| TaskError::RegexBuild {
        pattern: pattern.to_string(),
        error
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARKDOWN: &str = "## Methods\n\n\
        - [x] ready ([abc](https://example.com))\n\
        - [ ] brandNewMethod\n\n\
        ### WebAppInitData\n\
        - [x] hidden_field\n\n\
        ## Background events\n\n\
        | Event | Payload |\n\
        |-------|---------|\n\
        | `themeChanged` | none |\n";

    const SCRIPT: &str = "WebApp.ready = function () {};\n\
        Object.defineProperty(bottomButton, 'isVisible', { get: function(){ return true; } });\n\
        receiveWebViewEvent('themeChanged');\n\
        function onLocationRequested(eventType) {}\n\
        var Serverless = { call: function () {} };\n";

    #[test]
    fn checklist_items_collect_entries_and_events_and_skip_init_data_section() {
        let items = checklist_items(MARKDOWN).expect("regexes build");
        assert!(items.contains(&"ready".to_string()));
        assert!(items.contains(&"brandNewMethod".to_string()));
        assert!(items.contains(&"themeChanged".to_string()));
        assert!(!items.contains(&"hidden_field".to_string()));
    }

    #[test]
    fn checklist_items_without_entries_yields_nothing() {
        let items = checklist_items("# Title\n\nprose only\n").expect("regexes build");
        assert_eq!(items, Vec::<String>::new());
    }

    #[test]
    fn bridge_names_collects_assignments_properties_strings_and_functions() {
        let names = bridge_names(SCRIPT).expect("regexes build");
        for expected in [
            "ready",
            "isVisible",
            "themeChanged",
            "onLocationRequested",
            "call"
        ] {
            assert!(names.contains(expected), "missing {expected}");
        }
    }

    #[test]
    fn bridge_names_of_empty_script_is_empty() {
        assert!(bridge_names("").expect("regexes build").is_empty());
    }

    #[test]
    fn drift_is_detected_for_names_absent_from_the_bridge() {
        let items = checklist_items(MARKDOWN).expect("regexes build");
        let names = bridge_names(SCRIPT).expect("regexes build");
        let missing: Vec<&str> = items
            .iter()
            .map(String::as_str)
            .filter(|name| !names.contains(*name))
            .collect();
        assert_eq!(missing, ["brandNewMethod"]);
    }
}
