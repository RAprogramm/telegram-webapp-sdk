---
name: telegram-miniapp-rust
description: Build Telegram Mini Apps (WebApps) in Rust with telegram-webapp-sdk targeting wasm32-unknown-unknown. Covers the init flow, initData and TelegramContext, cloud/device/secure storage, Serverless endpoints, haptics and sensors, viewport, Yew and Leptos bindings, routing macros, mock-based local testing, and the official telegram-web-app.js page requirements.
---

# Telegram Mini Apps in Rust

Use this skill to build, test, and debug Telegram Mini Apps written in Rust
against `telegram-webapp-sdk` (wasm32-unknown-unknown).

## Non-negotiables

1. The page must load the official bridge before the app bundle:
   `<script src="https://telegram.org/js/telegram-web-app.js"></script>` in
   `<head>`. Without it `window.Telegram` is undefined and every call fails.
2. Call `init_sdk()` exactly once at startup, before touching
   `TelegramContext` or any API module. Its errors carry recovery hints —
   read them, do not retry blindly.
3. The app runs in a browser: compile to `wasm32-unknown-unknown`, serve via
   Trunk or any static server, open inside a Telegram client (or use the
   `mock` feature for local development).
4. Every SDK error message states what to do next. Surface the full message,
   never swallow it.

## Golden path

```rust
use telegram_webapp_sdk::{core::init::init_sdk, webapp::TelegramWebApp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    init_sdk()?;
    TelegramWebApp::instance()
        .ok_or_else(|| JsValue::from_str("Telegram WebApp not available"))?
        .ready()?;
    // mount UI here
    Ok(())
}
```

```toml
# Cargo.toml
[dependencies]
telegram-webapp-sdk = { version = "0.13", features = ["mock"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
```

```html
<!-- index.html (Trunk) -->
<head>
  <script src="https://telegram.org/js/telegram-web-app.js"></script>
</head>
```

## Choosing the right module

| Task | Module | Notes |
|---|---|---|
| Launch context, user, initData | `core::init`, `core::context::TelegramContext` | `try_init_sdk()` returns `Ok(false)` outside Telegram |
| Cross-device user data | `api::cloud_storage` | returns `js_sys::Promise`, await via `JsFuture` |
| Local device storage | `api::device_storage` | `async fn`, values per device |
| Encrypted storage with restore | `api::secure_storage` | `async fn`, needs biometric access |
| Bot backend calls | `api::serverless::call` | typed `EndpointError` with `hint()` |
| Vibration feedback | `api::haptic` | `impact_occurred`, `notification_occurred`, `selection_changed` |
| Motion sensors | `api::accelerometer`, `api::gyroscope`, `api::device_orientation` | start/stop pairs |
| Viewport, safe area, theme | `api::viewport`, `api::theme`, `webapp` facade | `TelegramWebApp::instance()` |
| Yew apps | `yew` feature | bottom button, back button, theme, viewport hooks |
| Leptos apps | `leptos` feature | same surface as reactive components/hooks |
| Page routing | `macros` feature | `telegram_page!`, `telegram_app!`, `router` |
| Develop outside Telegram | `mock` feature | `mock::mock_telegram_webapp(config)` |

## References

Read these when the task touches the area:

- `references/init-and-context.md` — init flow, error taxonomy, reading user
  and launch data, server-side initData validation.
- `references/storage.md` — the three storage backends, Promise vs async
  styles, error handling patterns.
- `references/serverless.md` — calling endpoints, typed payloads, decoding
  `EndpointError`, common platform error kinds.
- `references/frameworks.md` — Yew and Leptos components/hooks, executor
  setup for effects.
- `references/testing.md` — mock setup, wasm test rules (order, harness
  survival, deterministic yields).

## Scripts

- `scripts/scaffold.sh <dir>` — generate a minimal runnable vanilla mini app
  (Cargo.toml, Trunk.toml, index.html, src/main.rs) wired to the SDK.

## Debugging checklist

1. `window.Telegram is undefined` → bridge script missing or bundle loads
   before it.
2. `Telegram.WebApp is undefined` → wrong page context; the script tag must
   be in the HTML, not injected after wasm start.
3. `Failed to parse initData` → you re-encoded or hand-crafted initData; pass
   the raw string through.
4. Serverless call fails with `UNAUTHORIZED` → `init_sdk()` was not called
   first or initData expired; reopen the Mini App link.
5. Tests pass natively but hang in the browser → you awaited a non-Promise
   mock or navigated during a test; see `references/testing.md`.
