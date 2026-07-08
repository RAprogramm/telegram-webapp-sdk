# Release process

Releases are automated with [release-plz](https://release-plz.dev). There is no
manual version bump, changelog edit, or `cargo publish` step.

## How it works

1. **Land changes on `main`.** Merge feature/fix PRs as usual. Commit messages
   follow the `#<issue> <type>: <desc>` convention (`feat`, `fix`, `chore`,
   `docs`, `ci`, `refactor`, `test`).
2. **Release PR appears automatically.** On every push to `main`, the
   `Release-plz` workflow opens or updates a single **release PR** that:
   - bumps the version in `Cargo.toml` following semver (derived from the
     conventional commit types since the last tag — `feat` → minor,
     `fix`/others → patch, `!`/`BREAKING CHANGE` → major);
   - prepends the new section to `CHANGELOG.md`.
3. **Review and merge the release PR.** CI runs on it like any other PR.
   Merging it triggers the `release` job, which:
   - publishes `telegram-webapp-sdk` to crates.io;
   - creates the `vX.Y.Z` git tag;
   - creates the matching GitHub Release with the changelog section.

Only `telegram-webapp-sdk` is published. The `demo`, `vanilla-example`,
`webapp-bot-example`, and integration backend crates are marked
`publish = false` and are ignored by release-plz.

## One-time setup

The workflow authenticates with a `GH_TOKEN` repository secret (a PAT with
`contents:write` and `pull-requests:write` scopes) so that the release PR
triggers the normal CI checks. The `CARGO_REGISTRY_TOKEN` secret is used for
publishing to crates.io.

## Configuration

- `release-plz.toml` — release behavior and changelog grouping (mirrors
  `cliff.toml`).
- `.github/workflows/release-plz.yml` — the `release` and `release-pr` jobs.
