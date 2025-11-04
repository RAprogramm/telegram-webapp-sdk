<!--
    This README is automatically included on <https://docs.rs/telegram-webapp-sdk>.
    Keep sections concise and examples compilable where possible.
-->

<a id="readme-top"></a>

# Telegram WebApp SDK

[![Crates.io](https://img.shields.io/crates/v/telegram-webapp-sdk)](https://crates.io/crates/telegram-webapp-sdk)
[![docs.rs](https://img.shields.io/docsrs/telegram-webapp-sdk)](https://docs.rs/telegram-webapp-sdk)
[![Downloads](https://img.shields.io/crates/d/telegram-webapp-sdk)](https://crates.io/crates/telegram-webapp-sdk)
![MSRV](https://img.shields.io/badge/MSRV-1.89-blue)
![License](https://img.shields.io/badge/License-MIT-informational)
[![codecov](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk/graph/badge.svg?token=7FP6HC20BK)](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/telegram-webapp-sdk?branch=main)](https://hitsofcode.com/github/RAprogramm/telegram-webapp-sdk/view?branch=main)
[![CI](https://github.com/RAprogramm/telegram-webapp-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/RAprogramm/telegram-webapp-sdk/actions/workflows/ci.yml)
[![REUSE status](https://api.reuse.software/badge/github.com/RAprogramm/telegram-webapp-sdk)](https://api.reuse.software/info/github.com/RAprogramm/telegram-webapp-sdk)
<!-- webapp_api_badges:start -->
[![Telegram WebApp API](https://img.shields.io/badge/Telegram%20WebApp%20API-9.2-blue)](https://core.telegram.org/bots/webapps)
[![Coverage](https://img.shields.io/badge/Coverage-up%20to%20date%20%2892abbf7%29-brightgreen)](https://github.com/RAprogramm/telegram-webapp-sdk/commit/92abbf7)
<!-- webapp_api_badges:end -->
[![Wiki](https://img.shields.io/badge/Wiki-Documentation-0088cc?logo=github)](https://github.com/RAprogramm/telegram-webapp-sdk/wiki)

`telegram-webapp-sdk` provides a type-safe and ergonomic wrapper around the [Telegram Web Apps](https://core.telegram.org/bots/webapps) JavaScript API.

> [!NOTE]
> **Comprehensive Coverage**
>
> This project achieves comprehensive test coverage for both native and WASM code:
> - Native code coverage via `cargo-llvm-cov`
> - WASM code coverage via `wasmcov` with nightly toolchain
>
> Coverage reports include all modules (leptos, yew, api, webapp, logger, pages, router) ensuring quality across the entire codebase.
>
> For implementation details, see [issue #130](https://github.com/RAprogramm/telegram-webapp-sdk/issues/130).

<details>
<summary>Coverage Graphs</summary>

### Sunburst
The inner-most circle is the entire project, moving away from the center are folders then, finally, a single file. The size and color of each slice is representing the number of statements and the coverage, respectively.

[![Sunburst](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk/graphs/sunburst.svg?token=7FP6HC20BK)](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk)

### Grid
Each block represents a single file in the project. The size and color of each block is represented by the number of statements and the coverage, respectively.

[![Grid](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk/graphs/tree.svg?token=7FP6HC20BK)](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk)

### Icicle
The top section represents the entire project. Proceeding with folders and finally individual files. The size and color of each slice is representing the number of statements and the coverage, respectively.

[![Icicle](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk/graphs/icicle.svg?token=7FP6HC20BK)](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk)

</details>

## Table of contents

- [Coverage Graphs](#coverage-graphs)
- [Features](#features)
- [Macros](#macros)
- [Router](#router)
- [Installation](#installation)
- [Quick start](#quick-start)
  - [Yew](#yew)
  - [Leptos](#leptos)
- [Mock environment](#mock-environment)
- [User interactions](#user-interactions)
- [Keyboard control](#keyboard-control)
- [Closing confirmation](#closing-confirmation)
- [Invoice payments](#invoice-payments)
- [Sharing](#sharing)
- [Settings button](#settings-button)
- [Cloud storage](#cloud-storage)
- [Home screen](#home-screen)
- [Event callbacks](#event-callbacks)
  - [Background events](#background-events)
- [Appearance](#appearance)
- [Viewport](#viewport)
- [Fullscreen and orientation](#fullscreen-and-orientation)
- [Haptic feedback](#haptic-feedback)
- [Device storage](#device-storage)
- [Secure storage](#secure-storage)
- [Biometric authentication](#biometric-authentication)
- [Location manager](#location-manager)
- [Device sensors](#device-sensors)
- [Init data validation](#init-data-validation)
- [API coverage](#api-coverage)
- [Changelog](#changelog)
- [License](#license)
- [Metrics](#metrics)

<p align="right"><a href="#readme-top">Back to top</a></p>

## Features

- Comprehensive coverage of Telegram Web App JavaScript APIs.
- Framework integrations for **Yew** and **Leptos**.
- Optional macros for automatic initialization and routing.
- Biometric authentication helpers, viewport metrics, and theme utilities in
  step with the Telegram WebApp API 9.2 feature set.
  
<p align="right"><a href="#readme-top">Back to top</a></p>

## Macros

The macros are available with the `macros` feature. Enable it in your `Cargo.toml`:

```toml
telegram-webapp-sdk = { version = "0.2.15", features = ["macros"] }
```

Reduce boilerplate in Telegram Mini Apps using the provided macros:

```rust,ignore
telegram_page!("/", fn index() {
    // render page
});

telegram_app!(fn main() -> Result<(), wasm_bindgen::JsValue> {
    telegram_router!();
    Ok(())
});
```

When running outside Telegram in debug builds, `telegram_app!` loads mock
settings from `telegram-webapp.toml`.
- Configurable mock `Telegram.WebApp` for local development and testing.
- API helpers for user interactions, storage, device sensors and more.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Router

The `macros` feature ships with a minimal in-memory [`Router`](src/router.rs)
that collects pages registered via `telegram_page!`. The
[`telegram_router!`](src/macros.rs) macro builds this router and runs all page
handlers:

```rust,ignore
telegram_page!("/", pub fn index() {});

// Uses the default Router
telegram_router!();
```

Provide a custom router type to the macro if additional behavior is required:

```rust,ignore
struct CustomRouter;
impl CustomRouter {
    fn new() -> Self { CustomRouter }
    fn register(self, _path: &str, _handler: fn()) -> Self { self }
    fn start(self) {}
}

telegram_router!(CustomRouter);
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
telegram-webapp-sdk = "0.2"
```

Enable optional features as needed:

```toml
telegram-webapp-sdk = { version = "0.2.15", features = ["macros", "yew", "mock"] }
```

- `macros` &mdash; enables `telegram_app!`, `telegram_page!`, and `telegram_router!`.
- `yew` &mdash; exposes a `use_telegram_context` hook and a `BottomButton` component.
- `leptos` &mdash; integrates the context into the Leptos reactive system.
- `mock` &mdash; installs a configurable mock `Telegram.WebApp` for local development.
 
<p align="right"><a href="#readme-top">Back to top</a></p>

## Quick start

### Yew

```rust,ignore
use telegram_webapp_sdk::yew::use_telegram_context;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let ctx = use_telegram_context().expect("context");
    if let Some(query_id) = ctx.init_data.query_id.as_deref() {
        // Handle inline query response with `answerWebAppQuery`.
        let _ = query_id;
    }
    html! { <span>{ ctx.init_data.auth_date }</span> }
}
```

Use [`BottomButton`](https://docs.rs/telegram-webapp-sdk/latest/telegram_webapp_sdk/yew/struct.BottomButton.html) to control the main button:

```rust,ignore
use telegram_webapp_sdk::yew::BottomButton;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let on_click = Callback::from(|_| {});
    html! { <BottomButton text="Send" color="#000" text_color="#fff" {on_click} /> }
}
```

### Leptos

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::provide_telegram_context;

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let ctx = use_context::<telegram_webapp_sdk::core::context::TelegramContext>()
        .expect("context");
    if let Some(query_id) = ctx.init_data.query_id.as_deref() {
        // Handle inline query response with `answerWebAppQuery`.
        let _ = query_id;
    }
    view! { <span>{ ctx.init_data.auth_date }</span> }
}
```

The SDK also provides a `BottomButton` component for Leptos to control Telegram bottom buttons:

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::{provide_telegram_context, BottomButton};
use telegram_webapp_sdk::webapp::BottomButton as Btn;

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let (text, _set_text) = signal("Send".to_owned());
    view! { <BottomButton button=Btn::Main text /> }
}
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Mock environment

The `mock` feature simulates a `Telegram.WebApp` instance, enabling local development without Telegram:

```rust,ignore
let config = telegram_webapp_sdk::mock::MockConfig::default();
let ctx = telegram_webapp_sdk::mock::install(config)?;
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## User interactions

Request access to sensitive user data or open the contact interface:

```rust,no_run
use telegram_webapp_sdk::api::user::{request_contact, request_phone_number, open_contact};
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
request_contact()?;
request_phone_number()?;
open_contact()?;

let app = TelegramWebApp::try_instance()?;
app.request_write_access(|granted| {
    let _ = granted;
})?;
# Ok(())
# }
```

These calls require the user's explicit permission before any information is shared.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Keyboard control

Hide the native keyboard when it's no longer required:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.hide_keyboard()?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Closing confirmation

Prompt users before the Mini App closes:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.enable_closing_confirmation()?;
assert!(app.is_closing_confirmation_enabled());
// later
app.disable_closing_confirmation()?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Invoice payments

Open invoices and react to the final payment status:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let handle = app.on_invoice_closed(|status| {
    let _ = status;
})?;
app.open_invoice("https://invoice", |_status| {})?;
app.off_event(handle)?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Sharing

Share links, prepared messages, or stories and join voice chats:

```rust,no_run
use js_sys::Object;
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.share_url("https://example.com", Some("Check this out"))?;
app.join_voice_chat("chat", None)?;
app.share_message("msg-id", |sent| {
    let _ = sent;
})?;
let params = Object::new();
app.share_to_story("https://example.com/image.png", Some(&params.into()))?;
# Ok(())
# }
```
<p align="right"><a href="#readme-top">Back to top</a></p>
## Settings button

Control the Telegram client's settings button and handle user clicks:

```rust,no_run
use telegram_webapp_sdk::api::settings_button::{show, hide, on_click, off_click};
use wasm_bindgen::prelude::Closure;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
on_click(&cb)?;
show()?;
hide()?;
off_click(&cb)?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Cloud storage

Persist small key-value pairs in Telegram's cloud using `CloudStorage`:

```rust,no_run
use js_sys::Reflect;
use telegram_webapp_sdk::api::cloud_storage::{get_items, set_items};
use wasm_bindgen_futures::JsFuture;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
JsFuture::from(set_items(&[("counter", "1")])?).await?;
let obj = JsFuture::from(get_items(&["counter"])?).await?;
let value = Reflect::get(&obj, &"counter".into())?.as_string();
assert_eq!(value, Some("1".into()));
# Ok(())
# }
```

All functions return a `Promise` and require the Web App to run inside Telegram.
<p align="right"><a href="#readme-top">Back to top</a></p>
## Home screen

Prompt users to add the app to their home screen and check the current status:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let _shown = app.add_to_home_screen()?;
app.check_home_screen_status(|status| {
    let _ = status;
})?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Event callbacks

Callback registration methods return an `EventHandle` for later deregistration.

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let handle = app.on_event("my_event", |value| {
    let _ = value;
})?;
app.off_event(handle)?;
# Ok(())
# }
```

### Background events

Some Telegram events may fire while the Mini App is in the background. Register
callbacks for these with `on_background_event`:

```rust,no_run
use telegram_webapp_sdk::webapp::{BackgroundEvent, TelegramWebApp};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let handle = app.on_background_event(BackgroundEvent::MainButtonClicked, |_| {})?;
app.off_event(handle)?;
# Ok(())
# }
```

Supported background events:

| Event | Payload |
|-------|---------|
| `mainButtonClicked` | none |
| `backButtonClicked` | none |
| `settingsButtonClicked` | none |
| `writeAccessRequested` | `bool` granted flag |
| `contactRequested` | `bool` shared flag |
| `phoneRequested` | `bool` shared flag |
| `invoiceClosed` | status `String` |
| `popupClosed` | object `{ button_id: Option<String> }` |
| `qrTextReceived` | scanned text `String` |
| `clipboardTextReceived` | clipboard text `String` |

<p align="right"><a href="#readme-top">Back to top</a></p>

## Appearance

Customize colors and react to theme or safe area updates:

```rust,no_run
use telegram_webapp_sdk::api::theme::get_theme_params;
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.set_header_color("#0a0a0a")?;
app.set_background_color("#ffffff")?;
app.set_bottom_bar_color("#2481cc")?;

let params = get_theme_params()?;
let _ = params.bg_color;

let theme_handle = app.on_theme_changed(|| {
    let _ = get_theme_params();
})?;
let safe_handle = app.on_safe_area_changed(|| {})?;
let content_handle = app.on_content_safe_area_changed(|| {})?;

app.off_event(theme_handle)?;
app.off_event(safe_handle)?;
app.off_event(content_handle)?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Viewport

Inspect the Mini App viewport size and subscribe to updates:

```rust,no_run
use telegram_webapp_sdk::api::viewport::{
    expand_viewport, get_viewport_height, on_viewport_changed,
};
use wasm_bindgen::closure::Closure;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let _ = get_viewport_height();
let callback = Closure::wrap(Box::new(|| {
    let _ = get_viewport_height();
}) as Box<dyn Fn()>);
on_viewport_changed(&callback);
expand_viewport()?;
callback.forget();
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Fullscreen and orientation

Control the Mini App display and screen orientation:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
if !app.is_fullscreen() {
    app.request_fullscreen()?;
}
app.lock_orientation("portrait")?;
app.unlock_orientation()?;
app.exit_fullscreen()?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

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

<p align="right"><a href="#readme-top">Back to top</a></p>

## Device storage

Persist lightweight data on the user's device:

```rust,no_run
use telegram_webapp_sdk::api::device_storage::{set, get};

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
set("theme", "dark").await?;
let value = get("theme").await?;
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Secure storage

Store sensitive data encrypted and restorable:

```rust,no_run
use telegram_webapp_sdk::api::secure_storage::{set, restore};

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
set("token", "secret").await?;
let _ = restore("token").await?;
# Ok(())
# }
```
<p align="right"><a href="#readme-top">Back to top</a></p>
## Biometric authentication

Guard privileged actions behind the BiometricManager API:

```rust,no_run
use telegram_webapp_sdk::api::biometric::{
    authenticate, init, is_biometric_available, request_access,
};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
init()?;
if is_biometric_available()? {
    request_access("auth-key", Some("Unlock the vault"), None)?;
    authenticate("auth-key", None, None)?;
}
# Ok(())
# }
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Location manager

Retrieve user location and react to related events via Telegram's location manager:

```rust,no_run
use telegram_webapp_sdk::api::location_manager::{
    init, get_location, open_settings, on_location_requested,
};
use wasm_bindgen::closure::Closure;

init()?;
let _ = get_location();
open_settings()?;

let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
on_location_requested(&cb)?;
cb.forget();
# Ok::<(), wasm_bindgen::JsValue>(())
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Device sensors

Access motion sensors if the user's device exposes them.

```rust,no_run
use telegram_webapp_sdk::api::accelerometer::{start, get_acceleration, stop};

start()?;
let reading = get_acceleration();
stop()?;
# Ok::<(), wasm_bindgen::JsValue>(())
```

Callbacks for sensor lifecycle events are available through `on_started`,
`on_changed`, `on_stopped`, and `on_failed` functions for accelerometer,
gyroscope, and device orientation sensors.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Init data validation

### Retrieving raw initData

Retrieve the raw URL-encoded `initData` string for server-side authentication.
The SDK captures this string during initialization and provides convenient
access without requiring JavaScript reflection:

```rust,no_run
use telegram_webapp_sdk::TelegramWebApp;

# fn run() -> Result<(), Box<dyn std::error::Error>> {
// Get raw initData for backend validation
let raw_init_data = TelegramWebApp::get_raw_init_data()?;

// Send to your backend for signature verification
// POST /auth with body: { "init_data": raw_init_data }
# Ok(())
# }
```

This eliminates the need for manual `Reflect` calls and ensures consistency
with the parsed data available in the context.

### Validating initData

**Server-side validation is required.** Use the [`init-data-rs`](https://github.com/escwxyz/init-data-rs) crate for backend validation:

```rust,ignore
// On your backend server
use init_data_rs::{validate, InitData};

async fn authenticate(init_data_str: &str, bot_token: &str) -> Result<InitData, Box<dyn std::error::Error>> {
    // Validate with optional expiration time (in seconds)
    let init_data: InitData = validate(init_data_str, bot_token, Some(3600))?;
    Ok(init_data)
}
```

**Why server-side only?**
- Bot tokens must never be exposed to client-side code
- Validation requires secret keys that should remain on the server
- This follows industry-standard security practices

See the [init-data-rs documentation](https://docs.rs/init-data-rs) for complete usage examples.

<p align="right"><a href="#readme-top">Back to top</a></p>

## API coverage

<!-- webapp_api_summary:start -->
**WebApp API coverage:** version `9.2` matches the latest Telegram WebApp API release `9.2`. Synced in commit [92abbf7](https://github.com/RAprogramm/telegram-webapp-sdk/commit/92abbf7) (recorded on 2025-09-21).
<!-- webapp_api_summary:end -->

See [WEBAPP_API.md](./WEBAPP_API.md) for a checklist of supported Telegram WebApp JavaScript API methods and features.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for release notes.

<p align="right"><a href="#readme-top">Back to top</a></p>

## License

`telegram-webapp-sdk` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Metrics

![Metrics](https://github.com/RAprogramm/infra-metrics-insight-renderer/blob/main/metrics/telegram-webapp-sdk.svg)
<p align="right"><a href="#readme-top">Back to top</a></p>
