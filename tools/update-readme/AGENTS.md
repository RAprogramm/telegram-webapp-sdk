# AGENTS.md — tools/update-readme

Generator and validator for the managed sections of the root `README.md`
(MSRV badge, WebApp API badges and summary, crate version in examples).

## Rules

- Never edit the managed README sections by hand → edit the sources
  (`WEBAPP_API.md` `[webapp_api_status]` metadata, `Cargo.toml`) and run
  `cargo run -p update-readme` from the repository root, then commit
  `README.md`. CI fails on drift.
- The version probe fetches the official Bot API source over the network;
  when the probe is unreachable it falls back to the declared
  `covered_version` — never make the fallback a hard failure.
- Managed regions are delimited by `<!-- ...:start -->` / `<!-- ...:end -->`
  marker pairs; never rename the markers, README regeneration depends on
  them.
