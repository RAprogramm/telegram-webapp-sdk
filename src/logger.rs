// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#[cfg(debug_assertions)]
use web_sys::console;

/// Internal helper for styled log output.
#[cfg_attr(not(debug_assertions), allow(unused_variables))]
fn styled_log(level: &str, emoji: &str, color: &str, msg: &str) {
    #[cfg(debug_assertions)]
    {
        let prefix = format!("%c[SDK] {} {}", emoji, level.to_uppercase());
        let style = format!("color: {color}; font-weight: bold");
        console::log_3(&prefix.into(), &style.into(), &msg.into());
    }
}

/// Logs a success message (✅ Green).
pub fn success(msg: &str) {
    styled_log("success", "✅", "lightgreen", msg);
}

/// Logs an error message (❌ Red).
pub fn error(msg: &str) {
    styled_log("error", "❌", "red", msg);
}

/// Logs a warning message (⚠️ Orange).
pub fn warn(msg: &str) {
    styled_log("warn", "⚠️", "orange", msg);
}

/// Logs an info message (ℹ️ Blue).
pub fn info(msg: &str) {
    styled_log("info", "ℹ️", "#3399ff", msg);
}

/// Logs a debug message (🔧 Gray).
pub fn debug(msg: &str) {
    styled_log("debug", "🔧", "#888", msg);
}

/// Logs a trace message (📍 Light Gray).
pub fn trace(msg: &str) {
    styled_log("trace", "📍", "#aaa", msg);
}
