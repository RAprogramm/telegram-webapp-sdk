# AGENTS.md

Guidance for AI coding agents working in this repository. Read this file
fully before changing anything; nested `AGENTS.md` files add directory-local
rules on top of these.

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
xtask/              maintenance tasks in Rust: check-all, site (see below)
site/               docs-site landing page (the builder lives in xtask)
wiki/               docs-site source pages (synced to the GitHub Wiki)
WEBAPP_API.md       WebApp API coverage checklist + status metadata
```

Workspace members: `.`, `demo`, `examples/vanilla`, `examples/bots/rust_bot`,
`examples/integration/backend`, `tools/update-readme`, `xtask`.
`examples/integration/frontend` is excluded and has its own `Cargo.lock`.

## Cargo features

- `macros` — routing/page macros (`telegram_page!`, `telegram_app!`).
- `yew` — Yew bindings.
- `leptos` — Leptos bindings.
- `mock` — mock `Telegram.WebApp` for running outside Telegram in debug.
- `full` — all of the above.

## Commands

One command verifies everything CI verifies:

```bash
cargo run -p xtask -- check-all
```

Individual steps, when you need them separately:

```bash
cargo +nightly fmt                                  # formatting (nightly only)
cargo clippy --workspace --all-targets --all-features \
  -- -D warnings -W clippy::pedantic -W clippy::nursery
cargo nextest run --all-features                    # native tests
cargo test --doc --all-features                     # doctests (README included!)
cargo build --release --all-features
cargo bench                                         # Divan benchmarks
cargo run -p update-readme                          # regenerate README sections
cargo run -p xtask -- site --out /tmp/site          # build docs site (needs mdbook)
```

Wasm tests run in a real browser and are a blocking CI gate:

```bash
cargo test --target wasm32-unknown-unknown --all-features
```

Locally they need `geckodriver` on `PATH`, `FIREFOX_BIN` pointing at Firefox,
and a `wasm-bindgen-cli` version matching `Cargo.lock`
(`cargo binstall wasm-bindgen-cli@<lock-version>`).

`tools/update-readme` rewrites the managed README sections (MSRV badge,
WebApp API badges and summary, crate version in examples) from `WEBAPP_API.md`
metadata and `Cargo.toml`. CI fails if the README is out of sync — always run
the tool and commit the result after touching either source.

## Workflows

### Adding a Telegram.WebApp API surface

1. Implement the binding in `src/api/<area>.rs` (or `src/webapp/<area>.rs`
   for the facade), mirroring the existing modules: return
   `Result<Promise, JsValue>` or an `async fn` awaiting `JsFuture`.
2. Verify the exact JS signature against the official
   `https://telegram.org/js/telegram-web-app.js` — argument order matters.
3. Add tests in the same file: native negative scenarios plus wasm positive
   scenarios (see Testing rules below).
4. Add the entry to the `WEBAPP_API.md` checklist with a commit link and
   update the `[webapp_api_status]` metadata block (`covered_version`,
   `coverage_commit`, `coverage_date`).
5. Run `cargo run -p xtask -- check-all` and commit the result.

### Changing documentation

1. Edit pages under `wiki/` — never edit the GitHub Wiki in the browser;
   `wiki.yml` syncs `wiki/` to it.
2. Preview with `cargo run -p xtask -- site --out /tmp/site` (needs mdbook).
3. README badges and summary are generated — edit `WEBAPP_API.md` metadata
   and run `cargo run -p update-readme`, never the README sections directly.

### Before committing

1. `cargo run -p xtask -- check-all` must pass (fmt, clippy `-D warnings`,
   nextest, doctests, wasm check, README sync).
2. If you touched test code or anything browser-facing, also run the wasm
   suite locally.
3. Never commit red.

## Conventions

- Rust edition 2024; line limit 99 chars; braces on the same line.
- Never create `mod.rs` files → modules are `foo.rs` + optional `foo/`
  directory.
- Never call `unwrap()`/`expect()` outside tests → return
  `Result<T, JsValue>` or typed errors with manual `Display` and
  `std::error::Error` impls; tools and binaries use `masterror`.
- Never add `#[allow(...)]` suppressions → fix the lint exactly as clippy
  suggests.
- Never write comments inside function bodies → rename, extract, or
  restructure until the code speaks; documentation lives in `///`/`//!`
  doc blocks with purpose, arguments, return value, and a usage example.
- Never leave a public item undocumented (`#![warn(missing_docs)]` is
  enforced) → add a doc block with a ```` ```no_run ```` example when a
  browser is required.
- Never surface a bare failure from an error type → every error `Display`
  must state what went wrong and what the caller should do next (see
  `InitError` and `EndpointError::hint` for the established style).

## Testing rules

- Native unit tests live next to the code (`#[cfg(test)] mod tests`);
  browser tests use `wasm_bindgen_test` with
  `wasm_bindgen_test_configure!(run_in_browser)`.
- Tests must be deterministic — no network, wall-clock, or environment
  dependencies. Include negative scenarios for every error path.
- Never assume test order inside one test binary — `wasm-bindgen-test`
  runs tests in unspecified order. The global `TelegramContext` is a
  `OnceCell`, so exactly one test per binary may own the init lifecycle
  (uninitialized → initialized → already-initialized); split fresh-state
  scenarios into a separate binary.
- Never navigate in a wasm test (`location.search` assignment reloads and
  kills the harness) → mutate state with `history.replaceState`.
- Never render framework components into `document.body` in tests (it
  wipes the harness DOM) → append and render into a dedicated `div`.
- Never return raw values from JS storage mocks — the SDK awaits Promises →
  return `Promise.resolve(...)`.
- Never drop the `EventHandle` returned by event registration before the
  callback fires; the closure is dropped with it.
- Leptos effects need `any_spawner::Executor::init_wasm_bindgen()` before
  they run; yield microtasks (`JsFuture::from(Promise::resolve(...))`)
  instead of sleeping.

## Git

- Branch name = issue number; PR title = branch name; PR body contains
  `Closes #<issue>`.
- Commit format: `#<issue> type: description` (e.g. `#42 feat: add handler`).
- Never commit red: fmt, clippy, tests, and doctests must pass locally.
