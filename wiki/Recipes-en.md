# Recipes

Complete, copy-paste-ready solutions for the two tasks every production Mini
App eventually hits: proving on your server that a launch really came from
Telegram, and calling your backend through the Serverless bridge with typed
errors. Both recipes assume the SDK is already initialized with `init_sdk()`.

## Validate `initData` on the backend

The SDK never checks the `hash` field client-side: validation requires the bot
token, and the bot token must never reach the frontend. The frontend's only
job is to hand the untouched launch string to your backend:

```rust,ignore
use telegram_webapp_sdk::core::context::TelegramContext;

let raw_init_data =
    TelegramContext::get_raw_init_data().map_err(|error| JsValue::from_str(error))?;
// POST `raw_init_data` verbatim — do not re-encode or reorder the pairs.
```

On the backend (native Rust, not WASM), recompute the signature exactly the
way the Bot API specifies: build the data-check-string from the sorted
`key=value` pairs (minus `hash`), derive the secret key from the bot token,
and compare hashes in constant time.

```toml
# backend Cargo.toml
hmac = "0.12"
sha2 = "0.10"
percent-encoding = "2"
```

```rust,ignore
use hmac::{Hmac, Mac};
use percent_encoding::percent_decode_str;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Validates a raw `initData` string against the bot token.
///
/// Returns the parsed fields on success. Rejects launches older than
/// `max_age_secs` so captured payloads cannot be replayed.
pub fn validate_init_data(
    raw: &str,
    bot_token: &str,
    max_age_secs: u64,
) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    let mut received_hash = None;
    for pair in raw.split('&') {
        let (key, value) = pair.split_once('=').ok_or("malformed initData pair")?;
        let decoded = percent_decode_str(value)
            .decode_utf8()
            .map_err(|_| "initData value is not valid utf-8")?
            .into_owned();
        match key {
            "hash" => received_hash = Some(decoded),
            _ => {
                fields.insert(key.to_owned(), decoded);
            }
        }
    }
    let received_hash = received_hash.ok_or("initData has no hash field")?;

    let data_check_string = fields
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut key_mac =
        HmacSha256::new_from_slice(b"WebAppData").map_err(|_| "fixed key must be valid")?;
    key_mac.update(bot_token.as_bytes());
    let secret_key = key_mac.finalize().into_bytes();

    let mut mac = HmacSha256::new_from_slice(&secret_key).map_err(|_| "derived key invalid")?;
    mac.update(data_check_string.as_bytes());
    let signature = decode_hex(&received_hash).ok_or("hash is not hex-encoded")?;
    mac.verify_slice(&signature)
        .map_err(|_| "initData signature mismatch")?;

    let auth_date: u64 = fields
        .get("auth_date")
        .and_then(|value| value.parse().ok())
        .ok_or("initData has no auth_date")?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system clock is before the epoch")?
        .as_secs();
    if now.saturating_sub(auth_date) > max_age_secs {
        return Err("initData is too old, reject the launch".to_string());
    }
    Ok(fields)
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).ok())
        .collect()
}
```

Checklist:

- Compare with `verify_slice` (constant time), never with `==` on hex strings.
- Reject stale `auth_date` — a captured URL keeps working forever otherwise.
- Treat every field the frontend sends as untrusted until this function
  returns `Ok`; `user` parsed client-side is display-only.

## Typed Serverless calls with error handling

`api::serverless::call` serializes the input, awaits the bridge Promise and
deserializes the result, so one line covers the whole roundtrip:

```rust,ignore
use serde::{Deserialize, Serialize};
use telegram_webapp_sdk::api::serverless::{self, EndpointError};

#[derive(Serialize)]
struct BalanceInput {
    user_id: i64,
}

#[derive(Deserialize)]
struct Balance {
    amount: i64,
    currency: String,
}

async fn fetch_balance(user_id: i64) -> Result<Balance, EndpointError> {
    serverless::call("getBalance", &BalanceInput { user_id }).await
}
```

Failures arrive as a decoded [`EndpointError`](https://docs.rs/telegram-webapp-sdk/latest/telegram_webapp_sdk/api/serverless/struct.EndpointError.html)
with the platform `kind`, the HTTP `status` and optional `parameters`. Its
`Display` already appends a recovery hint, so logging the error is enough for
a human; branch on `kind` when the app can recover on its own:

```rust,ignore
async fn show_balance(user_id: i64) {
    match fetch_balance(user_id).await {
        Ok(balance) => log(&format!("{} {}", balance.amount, balance.currency)),
        Err(error) => match error.kind.as_str() {
            "UNAUTHORIZED" => restart_auth(),
            "TOO_MANY_REQUESTS" => schedule_retry(),
            _ => log(&error.to_string()),
        },
    }
}
```

- `kind` is empty and `status` is `0` for client-side failures (no bridge,
  payload that cannot be (de)serialized) — fix the caller, not the endpoint.
- `error.hint()` returns the same recovery hint as a `&'static str` when you
  need to map it programmatically.
- The endpoint itself must validate the launch (previous recipe) — Serverless
  forwards `initData`, it does not vouch for it.

## See also

- [Quick Start](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Quick-Start-en)
- [Examples](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/Examples-en)
- [WebApp API](https://github.com/RAprogramm/telegram-webapp-sdk/wiki/WebApp-API-en)
