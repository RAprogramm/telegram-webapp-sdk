# Quick Start

This page walks through the smallest useful Mini App: initialize the SDK, obtain
the WebApp instance, signal readiness, read the launch data, and show a MainButton.

## 1. Initialize the SDK and get the instance

`init_sdk()` parses `initData` and `themeParams` from `Telegram.WebApp` and
stores them in a global context. After that, obtain the live WebApp handle.

There are two accessors:

- `TelegramWebApp::instance()` returns `Option<TelegramWebApp>` (`None` when not
  running inside Telegram) — handy for graceful degradation.
- `TelegramWebApp::try_instance()` returns `Result<TelegramWebApp, JsValue>` —
  handy inside `?`-using functions.

```rust,ignore
use telegram_webapp_sdk::{core::init::init_sdk, webapp::TelegramWebApp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    init_sdk()?;

    let app = TelegramWebApp::try_instance()?;
    app.ready()?;   // tell Telegram the interface is initialized
    app.expand()?;  // expand the Mini App to full height

    Ok(())
}
```

If you prefer non-panicking startup, use `try_init_sdk()`, which returns
`Ok(false)` when Telegram is not present:

```rust,ignore
use telegram_webapp_sdk::core::init::try_init_sdk;

match try_init_sdk() {
    Ok(true) => { /* running inside Telegram */ }
    Ok(false) => { /* regular browser — use a fallback */ }
    Err(e) => eprintln!("init failed: {e}")
}
```

## 2. Read init data and the user

Parsed launch data lives in the global `TelegramContext`. Access it through the
closure-based getter:

```rust,ignore
use telegram_webapp_sdk::core::context::TelegramContext;

TelegramContext::get(|ctx| {
    if let Some(user) = &ctx.init_data.user {
        web_sys::console::log_1(&format!("Hello, {}", user.first_name).into());
    }
    let _ = ctx.init_data.auth_date;
    let _ = ctx.init_data.start_param.as_deref();
});
```

For server-side signature validation, grab the raw URL-encoded string and POST
it to your backend (validate it there with your bot token, never on the client):

```rust,ignore
use telegram_webapp_sdk::TelegramWebApp;

let raw_init_data = TelegramWebApp::get_raw_init_data()?;
// POST /auth  { "init_data": raw_init_data }
# Ok::<(), Box<dyn std::error::Error>>(())
```

## 3. A minimal MainButton

Set the label, show the button, and register a click callback. The callback
returns an `EventHandle` you can later pass to `remove_main_button_callback`.

```rust,ignore
use telegram_webapp_sdk::webapp::TelegramWebApp;

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

app.set_main_button_text("Send order")?;
app.set_main_button_color("#2481cc")?;
app.enable_main_button()?;
app.show_main_button()?;

let handle = app.set_main_button_callback(|| {
    if let Some(app) = TelegramWebApp::instance() {
        let _ = app.send_data("order-confirmed");
    }
})?;

// later, when the button is no longer needed:
app.remove_main_button_callback(handle)?;
app.hide_main_button()?;
# Ok(())
# }
```

## Where to next

- [WebApp API](WebApp-API-en) — the full covered surface, including dialogs, storage, and sensors
- [Framework Integration](Framework-Integration-en) — reactive hooks and button components for Leptos/Yew
- [Mock & Testing](Testing-en) — run this code outside Telegram
