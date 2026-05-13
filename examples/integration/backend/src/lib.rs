// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Message received from WebApp via sendData
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebAppMessage {
    /// Action type
    pub action:  String,
    /// Optional payload
    pub payload: Option<String>
}

/// Response sent to WebApp
#[derive(Debug, Serialize, Deserialize)]
pub struct WebAppResponse {
    /// Success status
    pub success: bool,
    /// Message to display
    pub message: String,
    /// Optional data
    pub data:    Option<String>
}
