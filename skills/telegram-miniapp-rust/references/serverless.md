# Serverless endpoints

`Telegram.WebApp.Serverless.call(name, input)` calls the bot's Serverless
endpoint deployed via BotFather / tgcloud. The platform attaches and verifies
initData — the endpoint sees a trusted user, the frontend sends no token.

## Typed call

```rust
use serde::{Deserialize, Serialize};
use telegram_webapp_sdk::api::serverless::{EndpointError, call};

#[derive(Serialize)]
struct ProfileInput {
    lang: String
}

#[derive(Deserialize)]
struct Profile {
    name: String
}

async fn load_profile() {
    match call::<ProfileInput, Profile>("getProfile", &ProfileInput { lang: "en".into() }).await {
        Ok(profile) => web_sys::console::log_1(&profile.name.into()),
        Err(err) => handle(err)
    }
}
```

`call_raw(name, &JsValue)` skips (de)serialization when the payload is
dynamic.

## Decoding failures

`EndpointError` fields: `message`, `kind` (platform category, may be empty),
`status` (HTTP code, `0` when the call never completed), `parameters`
(structured extras, e.g. endpoint `EndpointError` parameters or
`retry_after`). Its `Display` appends an actionable `hint()`.

```rust
fn handle(err: EndpointError) {
    web_sys::console::error_1(&err.to_string().into()); // message [KIND] (status) — hint
    match err.kind.as_str() {
        "ENDPOINT_ERROR" => { /* the endpoint refused; branch on codes inside err.parameters */ }
        "UNAUTHORIZED" => { /* initData missing/invalid: init_sdk() first, reopen the link */ }
        "TOO_MANY_REQUESTS" => { /* back off, honor retry_after in parameters */ }
        _ => { /* status class fallback: 5xx retry, 4xx fix the payload */ }
    }
}
```

## Rules

- Call `init_sdk()` before any Serverless call — the bridge sends initData
  from the initialized context.
- Endpoint names are JavaScript identifiers (`getProfile`, `place_order`).
- Never invent error categories: match `kind` against the platform values
  (`ENDPOINT_ERROR`, `UNAUTHORIZED`, `PAYMENT_REQUIRED`, `FORBIDDEN`,
  `NOT_FOUND`, `TOO_MANY_REQUESTS`) and fall back to `status`.
- If the app is not served from the bot's Serverless host, calls fail with
  status `0` — check the Mini App URL in BotFather.
