#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
# SPDX-License-Identifier: MIT

# Generate a minimal runnable Telegram Mini App skeleton wired to
# telegram-webapp-sdk. Usage: scaffold.sh <target-directory>

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <target-directory>" >&2
    exit 1
fi

DIR="$1"

if [[ -e "$DIR" ]]; then
    echo "error: '$DIR' already exists" >&2
    exit 1
fi

mkdir -p "$DIR/src"

cat > "$DIR/Cargo.toml" <<'EOF'
[package]
name = "my-miniapp"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
telegram-webapp-sdk = { version = "0.13", features = ["mock"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Window", "Document", "Element"] }
EOF

cat > "$DIR/Trunk.toml" <<'EOF'
[build]
target = "index.html"
dist = "dist"
EOF

cat > "$DIR/index.html" <<'EOF'
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>My Mini App</title>
    <script src="https://telegram.org/js/telegram-web-app.js"></script>
    <link data-trunk rel="rust" data-wasm-opt="z" />
  </head>
  <body>
    <main id="app">Loading…</main>
  </body>
</html>
EOF

cat > "$DIR/src/lib.rs" <<'EOF'
use telegram_webapp_sdk::{
    core::init::{init_sdk, is_telegram_available},
    mock::{config::MockTelegramConfig, init::mock_telegram_webapp},
    webapp::TelegramWebApp
};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    if !is_telegram_available() {
        mock_telegram_webapp(MockTelegramConfig::default())?;
    }
    init_sdk()?;

    let app = TelegramWebApp::instance()
        .ok_or_else(|| JsValue::from_str("Telegram WebApp not available"))?;
    app.ready()?;

    render(&app)?;
    Ok(())
}

fn render(app: &TelegramWebApp) -> Result<(), JsValue> {
    let document = window()
        .ok_or_else(|| JsValue::from_str("no window"))?
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;
    let main = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("no #app"))?;
    let name = TelegramWebApp::get_raw_init_data().unwrap_or_default();
    main.set_text_content(Some(&format!("SDK ready (raw initData length {})", name.len())));
    let _ = app.expand();
    Ok(())
}
EOF

echo "created $DIR — build with: cd $DIR && trunk serve"
