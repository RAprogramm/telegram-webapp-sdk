# Storage backends

Three backends, two call styles — do not mix them up.

## Cloud storage (`api::cloud_storage`) — Promise style

Per-user, synced across devices, scoped to the bot. Functions return
`js_sys::Promise`; await with `JsFuture`:

```rust
use telegram_webapp_sdk::api::cloud_storage;

async fn save(key: &str, value: &str) -> Result<(), wasm_bindgen::JsValue> {
    wasm_bindgen_futures::JsFuture::from(cloud_storage::set_item(key, value)?).await?;
    Ok(())
}

async fn load(key: &str) -> Result<Option<String>, wasm_bindgen::JsValue> {
    let value = wasm_bindgen_futures::JsFuture::from(cloud_storage::get_item(key)?).await?;
    Ok(value.as_string())
}
```

Surface: `get_item`, `set_item`, `remove_item`, `get_items`, `remove_items`,
`get_keys`.

## Device storage (`api::device_storage`) — async style

Local to the current device, not synced:

```rust
use telegram_webapp_sdk::api::device_storage;

device_storage::set_item("tab", "settings").await?;
let tab: Option<String> = device_storage::get_item("tab").await?;
device_storage::remove_item("tab").await?;
device_storage::clear().await?;
```

## Secure storage (`api::secure_storage`) — async style

Encrypted at rest; requires biometric access first
(`api::biometric::request_access` / `authenticate`):

```rust
use telegram_webapp_sdk::api::secure_storage;

secure_storage::set_item("token", secret).await?;
let stored = secure_storage::get_item("token").await?;
let token: Option<String> = stored.value;
let restored: Option<String> = secure_storage::restore_item("token").await?;
```

## Rules

- Never store secrets in cloud or device storage; only secure storage is
  encrypted.
- Treat `Ok(None)` from `get_item` as "absent", not an error.
- `secure_storage::get_item` returns `SecureStorageValue { value, can_restore }`;
  `can_restore` reports whether the client can restore the key from this device.
- Errors from the JS bridge carry the client's message; log the full
  `JsValue` string, it names the failing method.
- Cloud storage functions return a `js_sys::Promise` and test mocks must
  resolve it (`Promise.resolve(...)`); device and secure storage wrap the
  bridge callbacks `(err, value)` — mocks must invoke the callback with
  `null` on success and a string error otherwise.
