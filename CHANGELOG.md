# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
