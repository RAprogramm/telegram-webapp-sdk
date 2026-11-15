# AGENTS.md

Guidance for AI coding agents working in this repository.

## Project

`telegram-webapp-sdk` — a Rust SDK for building Telegram Mini Apps
(WebApps) targeting `wasm32-unknown-unknown`. Published on crates.io as
`telegram-webapp-sdk`; documents on docs.rs. MSRV is declared in
`Cargo.toml` (`[workspace.package].rust-version`).

## Repository layout

```
src/
  lib.rs            crate root; includes README as crate docs
  core/             init flow, global TelegramContext, wire types
    types/          serde models (init_data, user, chat, theme_params, ...)
  api/              one module per Telegram.WebApp sub-API
                    (cloud_storage, device_storage, secure_storage,
                    serverless, biometric, haptic, sensors, viewport, ...)
  webapp/           TelegramWebApp facade over the `Telegram.WebApp` object
  leptos/           Leptos components and hooks (`leptos` feature)
  yew/              Yew components and hooks (`yew` feature)
  mock/             mock Telegram.WebApp for local development (`mock`)
  dom/ utils/       DOM helpers and small utilities
  macros.rs router.rs pages.rs
                    routing macros (`macros` feature)
tests/              integration tests (native + wasm_bindgen_test)
benches/            Divan benchmarks (codspeed)
demo/ examples/     example apps (workspace members)
tools/update-readme README generator/validator (see below)
WEBAPP_API.md       WebApp API coverage checklist + status metadata
```

Workspace members: `.`, `demo`, `examples/vanilla`, `examples/bots/rust_bot`,
`examples/integration/backend`, `tools/update-readme`.
`examples/integration/frontend` is excluded and has its own `Cargo.lock`.

## Cargo features

- `macros` — routing/page macros (`telegram_page!`, `telegram_app!`).
- `yew` — Yew bindings.
- `leptos` — Leptos bindings.
- `mock` — mock `Telegram.WebApp` for running outside Telegram in debug.
- `full` — all of the above.

## Commands

```bash
cargo +nightly fmt                                  # formatting (nightly only)
cargo clippy --workspace --all-targets --all-features \
  -- -D warnings -W clippy::pedantic -W clippy::nursery
cargo nextest run --all-features                    # native tests
cargo test --doc --all-features                     # doctests (README included!)
cargo build --release --all-features
cargo bench                                         # Divan benchmarks
cargo run -p update-readme                          # regenerate README sections
```

Wasm tests (browser, informational in CI):

```bash
cargo test --target wasm32-unknown-unknown --all-features
```

`tools/update-readme` rewrites the managed README sections (MSRV badge,
WebApp API badges and summary, crate version in examples) from `WEBAPP_API.md`
metadata and `Cargo.toml`. CI fails if the README is out of sync — always run
the tool and commit the result after touching either source.

## Conventions

- Rust edition 2024; line limit 99 chars; braces on the same line.
- No `mod.rs` files — modules are `foo.rs` + optional `foo/` directory.
- Errors: `masterror::AppError` in tools/binaries; SDK API modules follow the
  established per-module error style (`JsValue` or dedicated typed errors with
  manual `Display`/`std::error::Error` impls).
- No `unwrap()`/`expect()` outside tests; no `#[allow(...)]` suppressions —
  fix lints as clippy suggests.
- No comments in function bodies; documentation lives in `///`/`//!` doc
  blocks with purpose, arguments, return value, and a usage example.
- Every public item requires docs (`#![warn(missing_docs)]` is enforced).
- Doc examples that need a browser use ```` ```no_run ````.
- API modules return `Result<Promise, JsValue>` (js_sys Promise) or `async fn`
  awaiting `JsFuture`, mirroring the existing storage modules.
- New Telegram.WebApp surface must be added to the `WEBAPP_API.md` checklist
  with a commit link, and the `[webapp_api_status]` metadata block updated
  (`covered_version`, `coverage_commit`, `coverage_date`).

## Testing

- Native unit tests live next to the code (`#[cfg(test)] mod tests`);
  browser tests use `wasm_bindgen_test` with
  `wasm_bindgen_test_configure!(run_in_browser)`.
- Tests must be deterministic — no network, wall-clock, or environment
  dependencies. Include negative scenarios for every error path.

## Git

- Branch name = issue number; PR title = branch name; PR body contains
  `Closes #<issue>`.
- Commit format: `#<issue> type: description` (e.g. `#42 feat: add handler`).
- Never commit red: fmt, clippy, tests, and doctests must pass locally.
