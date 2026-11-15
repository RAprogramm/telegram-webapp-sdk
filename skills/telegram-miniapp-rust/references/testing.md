# Testing Mini Apps

## Running outside Telegram: the mock feature

```rust
use telegram_webapp_sdk::mock::{config::MockTelegramConfig, init::mock_telegram_webapp};

if cfg!(debug_assertions) && !telegram_webapp_sdk::core::init::is_telegram_available() {
    mock_telegram_webapp(MockTelegramConfig::default())?;
    init_sdk()?;
}
```

`MockTelegramConfig` lets you set user, auth_date, hash, query_id, and theme
colors. Ship builds must not rely on the mock for production behavior.

## Native tests (`cargo nextest run --all-features`)

Pure logic: parsing, serialization, error formatting. `web_sys::window()`
panics on native targets — never test browser paths natively.

## Browser tests (`wasm_bindgen_test`, run_in_browser)

Hard rules learned the painful way:

1. **Order is unspecified.** The global `TelegramContext` is a `OnceCell`:
   exactly one test per binary may own the init lifecycle (uninitialized →
   initialized → already-initialized). Fresh-state scenarios go into a
   separate test binary.
2. **Never navigate.** Assigning `location.search`/`href` reloads the page
   and kills the harness (`logs is null`, "Failed to detect test"). Use
   `history.replace_state_with_url`.
3. **Never render into `document.body`.** Framework renders wipe the harness
   DOM; append a dedicated `div` and render into it.
4. **Mocks return Promises where the SDK awaits them** —
   `Function::new_with_args("k", "return Promise.resolve(this[k]);")`.
5. **Keep `EventHandle` alive** until the callback fires; dropping it drops
   the JS closure.
6. **No sleeping.** Yield microtasks deterministically:
   `JsFuture::from(Promise::resolve(&JsValue::NULL)).await` in a loop.
   Leptos effects additionally need
   `any_spawner::Executor::init_wasm_bindgen()`.

Local run:

```bash
# geckodriver on PATH, Firefox installed, wasm-bindgen-cli matching Cargo.lock
GECKODRIVER=$(command -v geckodriver) FIREFOX_BIN=/usr/bin/firefox \
  cargo test --target wasm32-unknown-unknown --all-features
```

## Debugging a hanging browser test

Run the harness interactively and open the served URL in a real browser:

```bash
NO_HEADLESS=1 WASM_BINDGEN_TEST_TIMEOUT=3600 \
  wasm-bindgen-test-runner target/wasm32-unknown-unknown/debug/build/<pkg>/out/<test>.wasm
```

The page console shows each invoked test and the panic that stopped the run.
