# Telegram WebApp SDK

`telegram-webapp-sdk` provides a type-safe and ergonomic wrapper around the [Telegram Web Apps](https://core.telegram.org/bots/webapps) JavaScript API.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
telegram-webapp-sdk = "0.1"
```

Optional features:

- `yew` &mdash; exposes a `use_telegram_context` hook.
- `leptos` &mdash; integrates the context into the Leptos reactive system.
- `mock` &mdash; installs a configurable mock `Telegram.WebApp` for local development.

Enable features as needed:

```toml
telegram-webapp-sdk = { version = "0.1", features = ["yew", "mock"] }
```

## Quick start

### Yew

```rust,no_run
use telegram_webapp_sdk::yew::use_telegram_context;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let ctx = use_telegram_context().expect("context");
    html! { <span>{ ctx.init_data.auth_date }</span> }
}
```

### Leptos

```rust,no_run
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::provide_telegram_context;

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let ctx = use_context::<telegram_webapp_sdk::core::context::TelegramContext>()
        .expect("context");
    view! { <span>{ ctx.init_data.auth_date }</span> }
}
```

## Mock environment

The `mock` feature simulates a `Telegram.WebApp` instance, enabling local development without Telegram:

```rust,no_run
let config = telegram_webapp_sdk::mock::MockConfig::default();
let ctx = telegram_webapp_sdk::mock::install(config)?;
```

## User interactions

Request access to sensitive user data or open the contact interface:

```rust,no_run
use telegram_webapp_sdk::api::user::{request_contact, request_phone_number, open_contact};

let _ = request_contact();
let _ = request_phone_number();
let _ = open_contact();
```

These calls require the user's explicit permission before any information is shared.

## Event callbacks

Callback registration methods return an `EventHandle` for later deregistration.

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
let app = TelegramWebApp::instance().unwrap();
let handle = app.on_event("my_event", |value| {
    let _ = value;
}).unwrap();
app.off_event(handle).unwrap();
```

## Haptic feedback

Trigger device vibrations through Telegram's [HapticFeedback](https://core.telegram.org/bots/webapps#hapticfeedback) API:

```rust,no_run
use telegram_webapp_sdk::api::haptic::{
    impact_occurred, notification_occurred, selection_changed,
    HapticImpactStyle, HapticNotificationType,
};

impact_occurred(HapticImpactStyle::Light)?;
notification_occurred(HapticNotificationType::Success)?;
selection_changed()?;
# Ok::<(), wasm_bindgen::JsValue>(())
```

## Init data validation

Validate the integrity of the `Telegram.WebApp.initData` payload on the server:

```rust
use telegram_webapp_sdk::utils::validate_init_data::{verify_hmac_sha256, verify_ed25519};

let bot_token = "123456:ABC";
let query = "user=alice&auth_date=1&hash=48f4c0e9d3dd46a5734bf2c5d4df9f4ec52a3cd612f6482a7d2c68e84e702ee2";
verify_hmac_sha256(query, bot_token)?;

// For Ed25519-signed data
# use ed25519_dalek::{Signer, SigningKey};
# let sk = SigningKey::from_bytes(&[1u8;32]);
# let pk = sk.verifying_key();
# let sig = sk.sign(b"a=1\nb=2");
# let init_data = format!("a=1&b=2&signature={}", base64::encode(sig.to_bytes()));
verify_ed25519(&init_data, pk.as_bytes())?;

# Ok::<(), Box<dyn std::error::Error>>(())
```

## API coverage

See [WEBAPP_API.md](./WEBAPP_API.md) for a checklist of supported Telegram WebApp JavaScript API methods and features.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for release notes.

## License

`telegram-webapp-sdk` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
