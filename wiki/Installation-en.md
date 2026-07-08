# Installation

## Add the dependency

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
telegram-webapp-sdk = "0.11"
```

## Enable feature flags

The crate ships with **no default features** (`default = []`), so you opt into
exactly what you need:

```toml
telegram-webapp-sdk = { version = "0.11", features = ["macros", "yew", "leptos", "mock"] }
```

| Feature  | Purpose |
|----------|---------|
| `macros` | `telegram_app!`, `telegram_page!`, and `telegram_router!` macros |
| `yew`    | Yew context hook, reactive hooks, and system-button components |
| `leptos` | Leptos context provider, reactive hooks, and system-button components |
| `mock`   | Configurable mock `Telegram.WebApp` for local development |
| `full`   | Shortcut for `macros` + `yew` + `leptos` + `mock` |

Typically you enable a single framework feature plus `mock`:

```toml
# Leptos app with local mock support
telegram-webapp-sdk = { version = "0.11", features = ["leptos", "mock"] }
```

## Minimum Supported Rust Version

The MSRV is **Rust 1.96** (edition 2024). Older toolchains are not supported.

```bash
rustup update stable
rustc --version   # must be >= 1.96
```

## Target the browser

Telegram Mini Apps run as WebAssembly in the Telegram in-app browser, so you
build for the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

## Bundling with Trunk or wasm-pack

Use a WASM bundler to produce the final `.wasm` + JS glue and an `index.html`.

**Trunk** (recommended for whole apps — this is what the demo uses):

```bash
cargo install trunk
trunk serve   # dev server with live reload
trunk build --release
```

A minimal `index.html` for Trunk pulls in Telegram's script and your crate:

```html
<!doctype html>
<html>
  <head>
    <script src="https://telegram.org/js/telegram-web-app.js"></script>
    <link data-trunk rel="rust" />
  </head>
  <body></body>
</html>
```

**wasm-pack** (for libraries or JS-driven integration):

```bash
cargo install wasm-pack
wasm-pack build --target web
```

## Next steps

- [Quick Start](Quick-Start-en) — initialize the SDK and show your first button
- [Framework Integration](Framework-Integration-en) — wire up Leptos or Yew
- [Mock & Testing](Testing-en) — run outside Telegram during development
