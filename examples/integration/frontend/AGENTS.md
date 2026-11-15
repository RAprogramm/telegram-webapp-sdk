# AGENTS.md — examples/integration/frontend

This crate is **excluded** from the cargo workspace and has its own
`Cargo.lock`; workspace-wide commands (`cargo run -p xtask -- check-all`,
`cargo clippy --workspace`) do not touch it.

## Build and check

```bash
cargo check --target wasm32-unknown-unknown        # type-check
trunk build                                       # full build (needs trunk)
```

## Local rules

- Never edit the root `Cargo.lock` from here and never edit this crate's lock
  from the workspace; they are independent.
- `rust-version` cannot inherit from `[workspace.package]` (excluded crate) →
  keep it manually in sync with the root `Cargo.toml`.
- The app consumes the SDK through the published crate path dependency with
  the `macros` feature; routing goes through the SDK's `telegram_page!` /
  `telegram_app!` macros, not hand-rolled routers.
