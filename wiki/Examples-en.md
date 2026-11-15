# Examples

The repository ships several runnable examples, wired into the Cargo workspace.
Each shows a different integration style — from a full Trunk-built Mini App down
to a plain WASM binary and the bot side of a full-stack app.

## `examples/minimal` — golden path

The canonical starting point: the shortest complete Mini App. It falls back to
the SDK mock when `Telegram.WebApp` is absent, calls `init_sdk()`, marks the
interface `ready()` + `expand()`, greets the launching user from the parsed
`TelegramContext`, and wires the MainButton to fire haptic feedback and
persist a visit counter through `DeviceStorage`.

```bash
cd examples/minimal
trunk serve        # http://localhost:8081
```

Read it top to bottom before writing your own app — every step it takes is one
the SDK docs explain in detail.

## `examples/serverless` — Mini App on Telegram Serverless

A full-stack demo whose backend runs on [Telegram
Serverless](https://core.telegram.org/bots/serverless) instead of a self-hosted
server: the Rust frontend calls the `getVisits` endpoint through
`api::serverless::call`, and the endpoint (a JavaScript module in
`tgcloud/endpoints/`) keeps a per-user counter in the platform's built-in
database. One `npm run deploy` ships frontend and backend together to
`https://app<app_id>.tgcloud.ai/`. The directory `README.md` walks through the
@BotFather setup.

```bash
cd examples/serverless
trunk serve          # local UI with the mock; http://localhost:8082
npm run deploy       # build + tgcloud push (after `npx tgcloud login`)
```

## `demo/` — Trunk WASM app

A complete Mini App built with the `macros` and `mock` features. It uses
`telegram_page!` / `telegram_router!` for routing and includes components and
pages (`demo/src/components`, `demo/src/pages`). Because it enables `mock`, it
runs in an ordinary browser during development.

```bash
cd demo
trunk serve        # dev server with live reload at http://localhost:8080
trunk build --release
```

The `demo/index.html` loads Telegram's `telegram-web-app.js` and the Trunk-built
WASM bundle.

## `examples/vanilla` — no framework

A pure-WebAssembly example (`examples/vanilla/src/main.rs` + `ui.rs`) that uses
the SDK and the DOM helpers directly, with no Yew or Leptos. It depends on the
crate with the `mock` feature, so it runs standalone. Good starting point if you
want full control over the DOM.

```bash
cd examples/vanilla
trunk serve
```

## `examples/bots/rust_bot` — Telegram bot

A [teloxide](https://github.com/teloxide/teloxide)-based bot
(`examples/bots/rust_bot`) that opens the demo Mini App via WebApp buttons and
receives orders sent from the app through `sendData`. This is the server side —
it is a native binary, not WASM.

```bash
cd examples/bots/rust_bot
cp .env.example .env          # add your TELOXIDE_TOKEN
cargo run
```

Requires a bot token from [@BotFather](https://t.me/BotFather) and an
HTTPS-served WebApp URL.

## `examples/integration` — full-stack

A two-part example showing the frontend↔backend round trip:

- `examples/integration/frontend` — a WASM Mini App that sends a JSON message to
  the bot via `TelegramWebApp::send_data(...)`. (This member is excluded from the
  workspace and built on its own with Trunk.)
- `examples/integration/backend` — a teloxide bot that receives the update's
  `web_app_data`, parses the JSON, and replies.

```bash
# terminal 1 — backend
cd examples/integration/backend
cargo run

# terminal 2 — frontend
cd examples/integration/frontend
trunk serve
```

The data flow: the user opens the Mini App → interacts with the UI → the app
calls `send_data` → Telegram delivers the payload to the bot as `web_app_data` →
the bot processes it and replies.

## Building any WASM example

All the browser-side examples build for `wasm32-unknown-unknown` and are served
with Trunk:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## See also

- [Quick Start](Quick-Start-en) — the minimal version of the vanilla example
- [Framework Integration](Framework-Integration-en) — Leptos/Yew patterns used in the demo
- [Mock & Testing](Testing-en) — the `mock` feature that lets these examples run outside Telegram
