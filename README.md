<!--
    This README is automatically included on <https://docs.rs/telegram-webapp-sdk>.
    Keep sections concise and examples compilable where possible.
-->

<a id="readme-top"></a>

# Telegram WebApp SDK

[![Crates.io](https://img.shields.io/crates/v/telegram-webapp-sdk)](https://crates.io/crates/telegram-webapp-sdk)
[![docs.rs](https://img.shields.io/docsrs/telegram-webapp-sdk)](https://docs.rs/telegram-webapp-sdk)
[![Downloads](https://img.shields.io/crates/d/telegram-webapp-sdk)](https://crates.io/crates/telegram-webapp-sdk)
<!-- msrv_badge:start -->
![MSRV](https://img.shields.io/badge/MSRV-1.95-blue)
<!-- msrv_badge:end -->
![License](https://img.shields.io/badge/License-MIT-informational)
[![codecov](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk/graph/badge.svg?token=7FP6HC20BK)](https://codecov.io/gh/RAprogramm/telegram-webapp-sdk)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/telegram-webapp-sdk?branch=main)](https://hitsofcode.com/github/RAprogramm/telegram-webapp-sdk/view?branch=main)
[![CI](https://github.com/RAprogramm/telegram-webapp-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/RAprogramm/telegram-webapp-sdk/actions/workflows/ci.yml)
[![REUSE status](https://api.reuse.software/badge/github.com/RAprogramm/telegram-webapp-sdk)](https://api.reuse.software/info/github.com/RAprogramm/telegram-webapp-sdk)
<!-- webapp_api_badges:start -->
[![Telegram WebApp API](https://img.shields.io/badge/Telegram%20WebApp%20API-9.6-blue)](https://core.telegram.org/bots/webapps)
[![Coverage](https://img.shields.io/badge/Coverage-up%20to%20date%20%2853276fd%29-brightgreen)](https://github.com/RAprogramm/telegram-webapp-sdk/commit/53276fd)
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
- **Vanilla WASM support** - use with any framework or none at all.
- Framework integrations for **Yew** and **Leptos** (optional).
- Optional macros for automatic initialization and routing.
- DOM helpers for ergonomic element manipulation.
- Biometric authentication helpers, viewport metrics, and theme utilities in
  step with the Telegram WebApp API 9.6 feature set.
  
<p align="right"><a href="#readme-top">Back to top</a></p>

## Macros

The macros are available with the `macros` feature. Enable it in your `Cargo.toml`:

```toml
telegram-webapp-sdk = { version = "0.9", features = ["macros"] }
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
telegram-webapp-sdk = "0.9"
```

Enable optional features as needed:

```toml
telegram-webapp-sdk = { version = "0.9", features = ["macros", "yew", "leptos", "mock"] }
```

- `macros` &mdash; enables `telegram_app!`, `telegram_page!`, and `telegram_router!`.
- `yew` &mdash; `use_telegram_context`, reactive hooks `use_viewport` / `use_theme` / `use_safe_area`, and components `BottomButton` / `BackButton` / `SettingsButton`.
- `leptos` &mdash; `provide_telegram_context`, same reactive `use_*` hooks and `BottomButton` / `BackButton` / `SettingsButton` components.
- `mock` &mdash; installs a configurable mock `Telegram.WebApp` for local development.
- `full` &mdash; aggregates `macros`, `yew`, `leptos`, `mock`.
 
<p align="right"><a href="#readme-top">Back to top</a></p>

## Quick start

### Vanilla (No Framework)

Use the SDK directly with pure WebAssembly - no framework required:

```rust,ignore
use telegram_webapp_sdk::{
    core::init::init_sdk,
    webapp::TelegramWebApp,
    dom::{Document, ElementExt},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    init_sdk()?;
    TelegramWebApp::instance()
        .ok_or_else(|| JsValue::from_str("Telegram not available"))?
        .ready()?;

    let doc = Document;

    let root = doc.create_element("div")?;
    root.set_class("container");

    let btn = doc.create_element("button")?;
    btn.set_text("Click me");
    btn.set_class("btn-primary");
    btn.on("click", |_| {
        web_sys::console::log_1(&"Clicked!".into());
    })?;
    root.append(&btn)?;

    doc.body()?.append(&root)?;

    Ok(())
}
```

#### DOM Helpers

The SDK includes ergonomic DOM manipulation helpers:

```rust,ignore
use telegram_webapp_sdk::dom::{Document, ElementExt};

// Get element by ID or selector
let el = Document.get_element_by_id("my-id");
let first = Document.query_selector(".item")?;

// Element manipulation
element.set_class("active");
element.set_id("unique-id");
element.set_text("Hello!");
element.set_html("<strong>Bold</strong>")?;
element.set_attr("data-value", "123")?;
element.remove_attr("data-value")?;

// Class manipulation
element.add_class("highlighted")?;
element.remove_class("hidden")?;
element.toggle_class("expanded")?;
let is_active = element.has_class("active");

// Event handling
element.on("click", |event| { /* handle click */ })?;
element.on("input", |event| { /* handle input */ })?;

// Tree manipulation
element.append(&child)?;
element.prepend(&header)?;
element.remove()?;        // detach self from parent
element.clear();          // remove all children
```

See [`examples/vanilla`](./examples/vanilla/) for a complete working example.

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

Yew also ships components for all three system buttons:

```rust,ignore
use telegram_webapp_sdk::yew::{BackButton, BottomButton, SettingsButton};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let on_main = Callback::from(|_| {});
    let on_back = Callback::from(|_| {});
    let on_settings = Callback::from(|_| {});
    html! {
        <>
            <BottomButton text="Send" color="#000" text_color="#fff" on_click={on_main} />
            <BackButton visible={true} on_click={on_back} />
            <SettingsButton visible={true} on_click={on_settings} />
        </>
    }
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

The SDK also provides `BottomButton`, `BackButton`, and `SettingsButton`
components for Leptos that drive the corresponding native Telegram buttons:

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::{
    provide_telegram_context, BackButton, BottomButton, SettingsButton
};
use telegram_webapp_sdk::webapp::BottomButton as Btn;

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let (text, _set_text) = signal("Send".to_owned());
    let back_visible = RwSignal::new(true);
    view! {
        <BottomButton button=Btn::Main text />
        <BackButton visible=back_visible on_click=move || { /* navigate back */ } />
        <SettingsButton visible=back_visible on_click=move || { /* open settings */ } />
    }
}
```

<p align="right"><a href="#readme-top">Back to top</a></p>

## Async API

Every one-shot Telegram callback has an `async fn` sibling that returns the
natural Rust type. Prefer `.await` for prod code; use the `*_with_callback`
variant when you can't `.await` (e.g. inside a non-async closure):

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

let confirmed: bool = app.show_confirm("Send the order?").await?;
let scanned: String = app.show_scan_qr_popup("Scan a QR code").await?;
let granted: bool = app.request_write_access().await?;
let _ = (confirmed, scanned, granted);
# Ok(())
# }
```

The same applies to `share_message`, `request_chat`, `check_home_screen_status`,
`set_emoji_status`, `request_emoji_status_access`, `open_invoice`,
`download_file`, `read_text_from_clipboard`, `show_popup`, and
`invoke_custom_method`.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Reactive hooks

Both Yew and Leptos integrations ship reactive hooks over Telegram's
state-changing events. The signals are seeded with the current values and
re-render the component when Telegram fires `viewportChanged`, `themeChanged`,
`safeAreaChanged`, or `contentSafeAreaChanged`. Cleanup is automatic on
unmount / scope disposal.

```rust,ignore
// Leptos
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::{use_safe_area, use_theme, use_viewport};

#[component]
fn Status() -> impl IntoView {
    let viewport = use_viewport();
    let theme = use_theme();
    let safe = use_safe_area();
    view! {
        <div>
            { move || viewport.get().height }
            { move || theme.get().color_scheme.unwrap_or_default() }
            { move || safe.get().area.map(|i| i.top).unwrap_or(0.0) }
        </div>
    }
}
```

```rust,ignore
// Yew
use telegram_webapp_sdk::yew::{use_safe_area, use_theme, use_viewport};
use yew::prelude::*;

#[function_component(Status)]
fn status() -> Html {
    let viewport = use_viewport();
    let theme = use_theme();
    let safe = use_safe_area();
    html! {
        <div>
            { viewport.height }
            { theme.color_scheme.clone().unwrap_or_default() }
            { safe.area.map(|i| i.top).unwrap_or(0.0) }
        </div>
    }
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

Request access to sensitive user data with the `async` API (preferred):

```rust,no_run
use telegram_webapp_sdk::api::user::request_contact;
use telegram_webapp_sdk::webapp::TelegramWebApp;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
request_contact()?;

let app = TelegramWebApp::try_instance()?;
let granted: bool = app.request_write_access().await?;
let sent: bool = app.request_chat(42).await?;
let _ = (granted, sent);
# Ok(())
# }
```

A synchronous callback variant is available as `*_with_callback` for code that
can't `.await` (e.g. `app.request_write_access_with_callback(|granted| { … })`).
All calls require the user's explicit permission before any information is
shared.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Keyboard control

Control the native keyboard and bottom buttons (Main and Secondary):

```rust,no_run
use telegram_webapp_sdk::webapp::{BottomButton, BottomButtonParams, TelegramWebApp};
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

// Hide the native keyboard
app.hide_keyboard()?;

// Control the main bottom button
app.set_main_button_text("Send")?;
app.set_main_button_color("#2481cc")?;
app.set_main_button_text_color("#ffffff")?;
app.enable_main_button()?;
app.show_main_button()?;

// Set custom emoji icon on the button (Bot API 9.5+)
app.set_main_button_icon_custom_emoji_id("123456789")?;

// Or use setParams for atomic updates
let params = BottomButtonParams {
    text: Some("Submit"),
    color: Some("#ff0000"),
    text_color: Some("#ffffff"),
    is_active: Some(true),
    is_visible: Some(true),
    icon_custom_emoji_id: Some("987654321"), // Bot API 9.5+
    ..Default::default()
};
app.set_main_button_params(&params)?;

// Secondary button (also supports icon_custom_emoji_id)
app.set_secondary_button_text("Cancel")?;
app.set_secondary_button_icon_custom_emoji_id("111222333")?;
app.show_secondary_button()?;
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

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let handle = app.on_invoice_closed(|status| {
    let _ = status;
})?;
let status: String = app.open_invoice("https://invoice").await?;
let _ = status;
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

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.share_url("https://example.com", Some("Check this out"))?;
let sent: bool = app.share_message("msg-id").await?;
let _ = sent;
let params = Object::new();
app.share_to_story("https://example.com/image.png", Some(&params.into()))?;
# Ok(())
# }
```
<p align="right"><a href="#readme-top">Back to top</a></p>
## Settings button

Control the Telegram client's settings button and handle user clicks
through the unified `TelegramWebApp` API:

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.show_settings_button()?;

let handle = app.set_settings_button_callback(|| {
    // user opened the settings menu
})?;

// when no longer needed:
app.remove_settings_button_callback(handle)?;
app.hide_settings_button()?;
# Ok(())
# }
```

The legacy standalone helpers in `api::settings_button` (`show`, `hide`,
`on_click`, `off_click`) remain available for callers that prefer the free
function style.

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
# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
let _shown = app.add_to_home_screen()?;
let status: String = app.check_home_screen_status().await?;
let _ = status;
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
| `invoiceClosed` | status `String` |
| `popupClosed` | object `{ button_id: Option<String> }` |
| `qrTextReceived` | scanned text `String` |
| `clipboardTextReceived` | clipboard text `String` |
| `requestedChatSent` | none (Bot API 9.6) |
| `requestedChatFailed` | object `{ error: String }` (Bot API 9.6) |

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
**WebApp API coverage:** version `9.6` matches the latest Telegram WebApp API release `9.6`. Bot API 9.5 added `icon_custom_emoji_id` for bottom buttons; 9.6 added `WebApp.requestChat` and the `requestedChatSent` / `requestedChatFailed` events.
<!-- webapp_api_summary:end -->

See [WEBAPP_API.md](./WEBAPP_API.md) for a checklist of supported Telegram WebApp JavaScript API methods and features.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for release notes.

<p align="right"><a href="#readme-top">Back to top</a></p>

## License

`telegram-webapp-sdk` is licensed under the MIT license — see
[`LICENSES/MIT.txt`](LICENSES/MIT.txt) or
<http://opensource.org/licenses/MIT>.

<p align="right"><a href="#readme-top">Back to top</a></p>

## Metrics

![Metrics](https://github.com/RAprogramm/infra-metrics-insight-renderer/blob/main/metrics/telegram-webapp-sdk.svg)
<p align="right"><a href="#readme-top">Back to top</a></p>
