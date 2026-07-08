// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Bindings for the individual Telegram WebApp JavaScript APIs.
//!
//! Each submodule wraps one area of the `Telegram.WebApp` interface, from
//! sensor access and haptic feedback to storage, theming, and viewport
//! information.

/// Accelerometer sensor: three-axis acceleration readings.
pub mod accelerometer;
/// Biometric manager: fingerprint/face authentication and access requests.
pub mod biometric;
/// Cloud storage: per-user key-value storage synced across devices.
pub mod cloud_storage;
/// Device orientation sensor: orientation angles in degrees.
pub mod device_orientation;
/// Device storage: local key-value storage on the current device.
pub mod device_storage;
/// WebApp event subscription helpers (`onEvent`/`offEvent`).
pub mod events;
/// Gyroscope sensor: angular velocity readings.
pub mod gyroscope;
/// Haptic feedback: impact, notification, and selection vibrations.
pub mod haptic;
/// Location manager: initialization and geolocation access.
pub mod location_manager;
/// Secure storage: encrypted key-value storage that survives reinstalls.
pub mod secure_storage;
/// Settings button: control over the WebApp settings button.
pub mod settings_button;
/// Theme parameters exposed by the Telegram client.
pub mod theme;
/// User data and contact/permission requests.
pub mod user;
/// Viewport dimensions and expansion state.
pub mod viewport;
