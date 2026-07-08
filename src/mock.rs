// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Mocking utilities for running the SDK outside a real Telegram client.
//!
//! Provides configuration, sample data, initialization helpers and utilities
//! to simulate the Telegram WebApp environment during local development and
//! testing.

/// Configuration types for describing the mocked Telegram environment.
pub mod config;
/// Sample data structures used to populate the mocked environment.
pub mod data;
/// Initialization helpers that install the mocked environment.
pub mod init;
/// Helper utilities for building and serializing mock data.
pub mod utils;
