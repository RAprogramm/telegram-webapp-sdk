// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Development task runner for the telegram-webapp-sdk workspace.
//!
//! Repository maintenance tasks that used to live in throwaway scripts live
//! here instead, so they are compiled, linted, and versioned together with
//! the SDK.
//!
//! Usage:
//!
//! ```text
//! cargo run -p xtask -- check-all
//! cargo run -p xtask -- site [--out DIR]
//! cargo run -p xtask -- help
//! ```

use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf}
};

use masterror::Error;

mod check_all;
mod conformance;
mod site;

/// Errors surfaced by the development tasks.
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("unknown command `{command}`; expected one of: check-all, conformance, site, help")]
    UnknownCommand { command: String },
    #[error("unknown option `{option}`")]
    UnknownOption { option: String },
    #[error("missing value for option `{option}`")]
    MissingOptionValue { option: String },
    #[error("environment variable CARGO_MANIFEST_DIR not set: {0}")]
    ManifestDir(env::VarError),
    #[error("could not locate the workspace root above the xtask package")]
    WorkspaceRootMissing,
    #[error("failed to spawn `{command}`: {error}")]
    Spawn {
        command: String,
        #[source]
        error:   std::io::Error
    },
    #[error("failed to {action} `{path}`: {error}")]
    Io {
        action: String,
        path:   String,
        #[source]
        error:  std::io::Error
    },
    #[error("failed to percent-decode `{value}`: {error}")]
    SlugDecode {
        value: String,
        error: std::str::Utf8Error
    },
    #[error("failed to build regex `{pattern}`: {error}")]
    RegexBuild {
        pattern: String,
        error:   regex::Error
    },
    #[error("languages missing from wiki/_Sidebar.md: {missing}")]
    LanguagesMissing { missing: String },
    #[error("failed to create a temporary working directory: {0}")]
    TempDir(std::io::Error),
    #[error("failed to spawn `mdbook` (install it with `cargo install mdbook`): {error}")]
    MdbookSpawn {
        #[source]
        error: std::io::Error
    },
    #[error("`mdbook build` for language `{lang}` exited with {status}")]
    MdbookFailed { lang: String, status: String },
    #[error("verification steps failed: {failed}")]
    ChecksFailed { failed: String },
    #[error("WEBAPP_API.md lists entries absent from the official bridge: {missing}")]
    ConformanceDrift { missing: String },
    #[error("failed to fetch the bridge script from {url}: {error}")]
    ConformanceFetch {
        url:   String,
        #[source]
        error: reqwest::Error
    }
}

fn main() -> Result<(), TaskError> {
    let mut args = env::args_os().skip(1);
    let command = args.next().map_or_else(
        || "help".to_string(),
        |arg| arg.to_string_lossy().into_owned()
    );
    match command.as_str() {
        "check-all" => check_all::run(),
        "conformance" => conformance::run(parse_conformance_js(args)?.as_deref()),
        "site" => site::run(parse_site_out(args)?.as_deref()),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        other => Err(TaskError::UnknownCommand {
            command: other.to_string()
        })
    }
}

/// Parses the optional `--out DIR` option of the `site` subcommand.
///
/// Returns the explicit output directory, or `None` when the caller should
/// fall back to the default location under `target/`.
fn parse_site_out<I: Iterator<Item = OsString>>(args: I) -> Result<Option<PathBuf>, TaskError> {
    let mut iter = args;
    let mut out: Option<PathBuf> = None;
    while let Some(arg) = iter.next() {
        let text = arg.to_string_lossy().into_owned();
        if text == "--out" {
            let value = iter.next().ok_or_else(|| TaskError::MissingOptionValue {
                option: "--out".to_string()
            })?;
            out = Some(PathBuf::from(value));
        } else {
            return Err(TaskError::UnknownOption {
                option: text
            });
        }
    }
    Ok(out)
}

/// Parses the optional `--js FILE` option of the `conformance` subcommand.
///
/// Returns the explicit bridge-script path, or `None` when the script should
/// be downloaded (with cache fallback).
fn parse_conformance_js<I: Iterator<Item = OsString>>(
    args: I
) -> Result<Option<PathBuf>, TaskError> {
    let mut iter = args;
    let mut js: Option<PathBuf> = None;
    while let Some(arg) = iter.next() {
        let text = arg.to_string_lossy().into_owned();
        if text == "--js" {
            let value = iter.next().ok_or_else(|| TaskError::MissingOptionValue {
                option: "--js".to_string()
            })?;
            js = Some(PathBuf::from(value));
        } else {
            return Err(TaskError::UnknownOption {
                option: text
            });
        }
    }
    Ok(js)
}

/// Returns the workspace root directory (the parent of the xtask package).
///
/// Relies on `CARGO_MANIFEST_DIR`, which cargo sets for the running binary.
pub fn repo_root() -> Result<PathBuf, TaskError> {
    let manifest = env::var("CARGO_MANIFEST_DIR").map_err(TaskError::ManifestDir)?;
    Path::new(&manifest)
        .parent()
        .map(Path::to_path_buf)
        .ok_or(TaskError::WorkspaceRootMissing)
}

/// Wraps an I/O failure with the action and path that produced it.
pub fn io_error(action: &str, path: &Path, error: std::io::Error) -> TaskError {
    TaskError::Io {
        action: action.to_string(),
        path: path.display().to_string(),
        error
    }
}

fn print_usage() {
    println!(
        "xtask — development tasks for telegram-webapp-sdk

Commands:
  check-all             run fmt, clippy, tests, doctests, wasm check, README sync
  conformance [--js F]  verify every WEBAPP_API.md entry exists in the official
                        telegram-web-app.js bridge (F: local bridge copy)
  site [--out DIR]      build the multilingual docs site from wiki/ with mdBook
                        (default output: <workspace>/target/site)
  help                  show this message
"
    );
}
