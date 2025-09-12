# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
