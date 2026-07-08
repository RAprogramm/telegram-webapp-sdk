# Mock & Testing

The `mock` feature installs a configurable fake `Telegram.WebApp` object on
`window`, so you can develop and test outside the Telegram client. Enable it:

```toml
telegram-webapp-sdk = { version = "0.11", features = ["mock"] }
```

## Installing a mock environment

The mock lives under `telegram_webapp_sdk::mock`. Build a `MockTelegramConfig`
(it implements `Default`) and call `mock_telegram_webapp`, which injects
`window.Telegram.WebApp` with mocked `initData`, `themeParams`, `platform`, and
`version`. Use it only in debug builds.

```rust,ignore
use telegram_webapp_sdk::mock::{config::MockTelegramConfig, init::mock_telegram_webapp};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let config = MockTelegramConfig::default();
mock_telegram_webapp(config)?;
// window.Telegram.WebApp now exists — the SDK behaves as if inside Telegram.
# Ok(())
# }
```

### Customizing the mocked data

`MockTelegramConfig` exposes optional fields for the user, auth data, every
theme color, and the platform/version. A mocked user is a `MockTelegramUser`:

```rust,ignore
use telegram_webapp_sdk::mock::{
    config::MockTelegramConfig, data::MockTelegramUser, init::mock_telegram_webapp,
};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let config = MockTelegramConfig {
    user: Some(MockTelegramUser {
        id: 42,
        first_name: "Alice".into(),
        username: Some("alice".into()),
        language_code: Some("en".into()),
        ..Default::default()
    }),
    platform: Some("android".into()),
    version: Some("9.6".into()),
    ..Default::default()
};
mock_telegram_webapp(config)?;
# Ok(())
# }
```

You can also load the config from a TOML file (the same `telegram-webapp.toml`
that `telegram_app!` reads in debug builds):

```rust,ignore
use telegram_webapp_sdk::mock::config::MockTelegramConfig;

let config = MockTelegramConfig::from_file("telegram-webapp.toml")?;
# Ok::<(), std::io::Error>(())
```

## Testing with `wasm_bindgen_test`

Because the SDK talks to browser globals, tests run in a headless browser via
`wasm-bindgen-test`. Add it as a dev-dependency:

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
```

Configure the tests to run in a browser and install the mock before exercising
the SDK:

```rust,ignore
use telegram_webapp_sdk::{
    core::init::init_sdk,
    mock::{config::MockTelegramConfig, init::mock_telegram_webapp},
    webapp::TelegramWebApp,
};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn instance_is_available_after_mock() {
    mock_telegram_webapp(MockTelegramConfig::default()).expect("mock installed");
    init_sdk().expect("sdk initialized");

    let app = TelegramWebApp::instance().expect("instance");
    app.ready().expect("ready");
}
```

Run the suite against a real browser engine:

```bash
# Chrome / Chromium
wasm-pack test --headless --chrome

# or Firefox
wasm-pack test --headless --firefox
```

## Tips

- Prefer `TelegramWebApp::instance()` (returning `Option`) in app code so the
  same binary degrades gracefully when neither Telegram nor the mock is present.
- Each `wasm_bindgen_test` shares one `window`; install the mock at the start of
  every test that needs it rather than relying on ordering.
- Note that `TelegramContext::init` succeeds only once per thread — design tests
  so they do not depend on re-initializing the global context.

## See also

- [Quick Start](Quick-Start-en) — the code you are testing
- [Examples](Examples-en) — the `examples/vanilla` app builds with the `mock` feature
