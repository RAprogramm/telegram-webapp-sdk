# AGENTS.md — xtask

Maintenance tasks for the repository, written in Rust. Repository automation
goes here — never add throwaway Python/Shell scripts.

## Commands

```bash
cargo run -p xtask -- check-all           # full local verification suite
cargo run -p xtask -- site --out <dir>    # build the EN/RU docs site (mdBook)
cargo run -p xtask -- help
```

## Rules

- One subcommand per module (`check_all.rs`, `site.rs`); add a new task as a
  new module plus a match arm in `main.rs`, with a `///` doc block.
- Errors: one `TaskError` enum in `main.rs` with `masterror` derive; every
  variant message must say what failed and where.
- Never `unwrap()`/`expect()` — propagate through `TaskError`.
- `site` must keep producing byte-identical mdBook output to the retired
  `site/build.py` (mdBook 0.5.3): same SUMMARY structure, same link
  rewriting, same landing page assembly. `llms.txt` at the output root is an
  intentional addition on top of the Python builder's output. Verify with a
  fresh `mdbook` build before changing it.
- `check-all` mirrors CI: every step runs even after a failure and the final
  error lists all failed steps. Keep the step list in sync with
  `.github/workflows/reusable-ci.yml`.
