// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::Deserialize;

/// Raw, minimally-typed view of the Telegram WebApp `initData` payload.
///
/// The complex fields (`user`, `receiver`, `chat`) are kept as their original
/// JSON-encoded strings so that the exact bytes can be used to verify the
/// `hash` signature before they are parsed into richer types. This mirrors the
/// structure of Telegram's `WebAppInitData` object as received on the client.
#[derive(Deserialize)]
pub struct TelegramInitDataInternal {
    /// Unique query identifier of the Mini App session. Present when the app is
    /// opened from an inline keyboard button and required to answer via
    /// `answerWebAppQuery`.
    pub query_id:       Option<String>,
    /// JSON-encoded string describing the current user, as delivered by
    /// Telegram. Deserialized into a `WebAppUser` only after validation.
    pub user:           Option<String>,
    /// JSON-encoded string describing the chat partner in a private chat the
    /// bot was attached to when launched from the attachment menu.
    pub receiver:       Option<String>,
    /// JSON-encoded string describing the chat the Mini App was launched from,
    /// present for group, supergroup and channel chats.
    pub chat:           Option<String>,
    /// Type of chat the Mini App was opened in: `"sender"`, `"private"`,
    /// `"group"`, `"supergroup"` or `"channel"`.
    pub chat_type:      Option<String>,
    /// Global identifier of the chat the Mini App was launched from, used to
    /// validate that messages belong to the same chat.
    pub chat_instance:  Option<String>,
    /// Value of the `startattach`/`start_param` deep-link parameter used to
    /// launch the Mini App.
    pub start_param:    Option<String>,
    /// Number of seconds after which a message can be sent via
    /// `answerWebAppQuery`, used for rate limiting.
    pub can_send_after: Option<u64>,
    /// Unix timestamp (in seconds) at which the `initData` was created and
    /// signed by Telegram.
    pub auth_date:      u64,
    /// Hex-encoded HMAC-SHA256 signature of the data-check string, used to
    /// verify that the `initData` originates from Telegram.
    pub hash:           String,
    /// Optional Ed25519 signature of the `initData`, provided for third-party
    /// validation of the payload.
    pub signature:      Option<String>
}
