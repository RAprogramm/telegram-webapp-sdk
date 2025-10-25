# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.16] - 2025-10-25
### Fixed
- Made `use_telegram_context` Yew hook reactive to properly handle cases when
  the Telegram SDK initializes after component mount. The hook now uses
  `requestAnimationFrame` to efficiently poll for context availability and
  automatically updates when the context becomes ready. ([#141](https://github.com/RAprogramm/telegram-webapp-sdk/issues/141), [#142](https://github.com/RAprogramm/telegram-webapp-sdk/pull/142))

## [0.2.15] - 2025-10-05
### Fixed
- Restored compatibility with the latest nightly toolchains by probing support
  for `doc_cfg` and `doc_auto_cfg`, ensuring docs.rs builds succeed after the
  upstream removal of `doc_auto_cfg`.

### Changed
- Emitted explicit `cargo:rustc-check-cfg` declarations for both documentation
  capability flags so that future compilers surface helpful diagnostics when
  the build script conditions fall out of sync.

## [0.2.14] - 2025-10-03
### Fixed
- Guarded the nightly-only `doc_auto_cfg` attribute behind a compiler channel
  probe so documentation builds on docs.rs no longer fail on stable toolchains.

### Changed
- Added a lightweight build-time check using `version_check` to conditionally
  enable nightly-only documentation features without impacting stable
  consumers.

## [0.2.13] - 2025-10-01
### Changed
- Upgraded to `masterror` 0.24 across the workspace, ensuring the SDK and demo
  benefit from the latest error handling improvements.

## [0.2.12] - 2025-09-24
### Changed
- Centralized the `masterror` 0.11 dependency in workspace metadata so all
  members, including the demo, use the same error handling crate without
  lingering `thiserror` references.

## [0.2.11] - 2025-09-23
### Changed
- Upgraded to `masterror` 0.11 across the workspace, fully removing any
  remaining reliance on `thiserror` derives.

## [0.2.10] - 2025-09-22
### Added
- Propagated `query_id` from `initData` into `TelegramInitData` and exposed it
  through the global context, including a wasm integration test that verifies
  the parsed value.

### Changed
- Extended the mock environment and documentation to surface the optional
  `query_id` parameter, ensuring examples highlight how to handle inline query
  responses.

## [0.2.9] - 2025-09-22
### Fixed
- Corrected launch parameter parsing to honor the first query entry,
  percent-decode values (including `tgWebAppPlatform`), and preserve
  boolean flags.

### Added
- Regression tests covering decoded query parameters for `get_launch_params`.

## [0.2.8] - 2025-09-21
### Changed
- Expanded the README with appearance, viewport, and biometric examples while
  updating dependency snippets to version `0.2.8`.
- Clarified the documented WebApp API coverage in `WEBAPP_API.md` and recorded
  the refreshed verification date.

## [0.2.7] - 2025-09-21
### Changed
- Removed the crate's direct dependency on `thiserror`, relying on `masterror`
  for error derives exclusively.

## [0.2.6] - 2025-09-21
### Changed
- Upgraded to `masterror` 0.10 and refreshed internal tooling error handling to
  accommodate the derive updates.

## [0.2.5] - 2025-09-20
### Changed
- Replaced `thiserror` derives with the new `masterror::Error` re-export and
  upgraded to `masterror` 0.5 for macro support.

## [0.2.4] - 2025-09-20
### Added
- Introduced `deny.toml` to codify `cargo deny` policies for advisories, license
  vetting, and source validation so security checks can run reproducibly.

### Changed
- Raised crate version to `0.2.4`.

## [0.2.3] - 2025-09-20
### Added
- Support for additional bottom button capabilities (`enable`, `disable`,
  `showProgress`, `hideProgress`, `setParams`, state accessors) covering the
  latest Telegram WebApp SDK behavior.
- New `BottomButtonParams` and `SecondaryButtonParams` helpers for ergonomic
  parameter updates, plus `SecondaryButtonPosition` and safe area accessors.
- `TelegramWebApp` utilities for safe area queries, fullscreen/activity state,
  vertical swipe detection and version checks.
- Optional `open_link` configuration via `OpenLinkOptions`.

### Changed
- Raised crate version to `0.2.3`.
- Updated documentation to reflect expanded API coverage.

## [0.2.2] - 2025-09-20
### Changed
- `update_readme` now discovers the latest Telegram WebApp API version directly
  from the upstream Bot API source, ensuring badges and summaries always report
  the current release.
- `WEBAPP_API.md` metadata includes an explicit probe URL for version
  discovery and tracks the latest Bot API release 9.2.

## [0.2.1] - 2025-09-20
### Added
- `update_readme` maintenance tool to sync Telegram WebApp API version badges
  and summary in the README from `WEBAPP_API.md` metadata.

### Changed
- README now displays Telegram WebApp API coverage status and the commit that
  implements the tracked version.

## [0.2.0] - 2025-09-12
### Changed
- Integrated macros into the main crate; `telegram-webapp-sdk-macros` crate removed.
- Replaced attribute macros with declarative macros `telegram_app!`, `telegram_page!`, and `telegram_router!`.

## [0.1.1] - 2025-09-12
### Added
- Implemented `CloudStorage.setItems`.

## [0.1.0] - 2025-09-12
### Added
- Initial release with core WebApp utilities, Yew and Leptos integrations,
  mock environment, and basic Bot API type definitions.
- User API wrappers: `request_contact`, `request_phone_number`, and `open_contact`.
- Accelerometer, gyroscope, and device orientation sensor APIs with start/stop,
  value reading and event callbacks.
- Home screen utilities: `add_to_home_screen` and `check_home_screen_status`.
