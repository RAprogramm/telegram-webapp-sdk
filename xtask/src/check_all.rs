// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Full local verification suite mirroring the CI pipeline.

use std::process::Command;

use crate::TaskError;

const STEPS: &[(&str, &str, &[&str])] = &[
    ("rustfmt", "cargo", &["+nightly", "fmt", "--all", "--check"]),
    (
        "clippy",
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings"
        ]
    ),
    (
        "tests",
        "cargo",
        &["nextest", "run", "--workspace", "--all-features"]
    ),
    (
        "doctests",
        "cargo",
        &["test", "--doc", "--workspace", "--all-features"]
    ),
    (
        "wasm check",
        "cargo",
        &[
            "check",
            "--target",
            "wasm32-unknown-unknown",
            "--all-features"
        ]
    ),
    ("readme refresh", "cargo", &["run", "-p", "update-readme"]),
    ("readme sync", "git", &["diff", "--exit-code", "README.md"])
];

/// Runs every verification step and reports all failures at once.
///
/// Each step runs even when an earlier one failed, so a single invocation
/// shows the complete state of the workspace. The README sync step refreshes
/// `README.md` via `update-readme` and then fails when the file changed,
/// mirroring the CI job.
///
/// Returns [`TaskError::ChecksFailed`] listing every failed step, or `Ok(())`
/// when all steps pass.
///
/// Example: `cargo run -p xtask -- check-all`.
pub fn run() -> Result<(), TaskError> {
    let mut failed: Vec<&str> = Vec::new();
    for &(name, program, args) in STEPS {
        println!("==> {name}");
        let status = Command::new(program)
            .args(args)
            .spawn()
            .map_err(|error| TaskError::Spawn {
                command: (*program).to_string(),
                error
            })?
            .wait()
            .map_err(|error| TaskError::Spawn {
                command: (*program).to_string(),
                error
            })?;
        if !status.success() {
            println!("FAIL: {name} exited with {status}");
            failed.push(name);
        }
    }
    if failed.is_empty() {
        println!("check-all: all steps passed");
        return Ok(());
    }
    Err(TaskError::ChecksFailed {
        failed: failed.join(", ")
    })
}
