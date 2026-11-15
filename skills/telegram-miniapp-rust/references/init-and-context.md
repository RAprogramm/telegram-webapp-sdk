# Init flow and context

## Startup sequence

```rust
use telegram_webapp_sdk::core::init::{init_sdk, is_telegram_available, try_init_sdk};

fn boot() -> Result<(), Box<dyn std::error::Error>> {
    // Strict: fails with a hint-carrying InitError outside Telegram.
    init_sdk()?;

    // Or lenient for code that also runs in a plain browser:
    // match try_init_sdk()? {
    //     true => {}                    // initialized inside Telegram
    //     false => {}                   // not Telegram; degrade gracefully
    //     Err(err) => return Err(err.into()), // broken initData; err tells why
    // }
    Ok(())
}
```

`init_sdk()` parses `Telegram.WebApp.initData`, `themeParams`, and launch
parameters into the global `TelegramContext`. It may run **once per page
load**; a second call fails with `ContextInitFailed` — reuse the context
instead of re-initializing.

## Reading the context

```rust
use telegram_webapp_sdk::core::context::TelegramContext;

let user_id = TelegramContext::get(|ctx| ctx.init_data.user.as_ref().map(|u| u.id));
let raw = TelegramContext::get_raw_init_data()?; // original URL-encoded string
```

`TelegramContext::get` returns `None` before initialization. The context
exposes `init_data` (user, chat, start_param, auth_date, query_id,
`chat_join_request_query_id`), `theme_params`, and `raw_init_data`.

## Error taxonomy (InitError)

Every variant's `Display` includes the recovery hint; match on the variant
only when you need programmatic branching:

- `WindowUnavailable` — no DOM window (worker/native); run in a page.
- `TelegramUnavailable` — no `window.Telegram`; open inside Telegram or use
  the `mock` feature.
- `WebAppUnavailable` — bridge script missing from the page.
- `InitDataParseFailed` / `ThemeParamsParseFailed` — malformed payload from
  the client; do not re-encode, pass through.
- `ContextInitFailed` — double init; reuse `TelegramContext::get()`.

## Server-side initData validation

The SDK never validates the `hash` client-side (the bot token must never
reach the frontend). Send `raw_init_data` to your backend and verify there:

1. Split off `hash`, build the data-check-string from the remaining
   key=value lines sorted lexicographically.
2. Derive the key as `HMAC_SHA256(key = "WebAppData", msg = bot_token)`.
3. Compare `HMAC_SHA256(key, data_check_string)` with `hash` (hex), and
   reject stale `auth_date` values.

For Serverless-backed apps this step is unnecessary: the platform verifies
initData before your endpoint runs.
