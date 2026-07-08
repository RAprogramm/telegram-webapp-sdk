# WebApp API

Almost every capability hangs off the `TelegramWebApp` handle
(`instance()` / `try_instance()`), with a few free-function modules under
`telegram_webapp_sdk::api::*` for sensors, storage, haptics, and theme. This
page groups the surface by area. For the exhaustive checklist, see
[`WEBAPP_API.md`](https://github.com/RAprogramm/telegram-webapp-sdk/blob/main/WEBAPP_API.md).

## The `*_with_callback` vs `async` pattern

Every one-shot Telegram callback has **two** Rust siblings:

- `foo_with_callback(..., F)` — synchronous registration; your closure runs when
  Telegram fires the result. Use it when you cannot `.await` (e.g. inside a
  non-async closure).
- `async fn foo(...)` — returns the natural Rust type via `.await`. Prefer this
  in production code.

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

The same dual applies to `share_message`, `request_chat`,
`check_home_screen_status`, `set_emoji_status`, `request_emoji_status_access`,
`open_invoice`, `download_file`, `read_text_from_clipboard`, `show_popup`, and
`invoke_custom_method`.

## Buttons

Main and Secondary bottom buttons share a `BottomButton` selector enum
(`BottomButton::Main` / `BottomButton::Secondary`):

```rust,no_run
use telegram_webapp_sdk::webapp::{BottomButton, BottomButtonParams, TelegramWebApp};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

app.set_bottom_button_text(BottomButton::Main, "Submit")?;
app.show_bottom_button(BottomButton::Main)?;

// Atomic update via setParams:
let params = BottomButtonParams {
    text: Some("Submit"),
    color: Some("#2481cc"),
    text_color: Some("#ffffff"),
    is_active: Some(true),
    is_visible: Some(true),
    icon_custom_emoji_id: Some("123456789"), // Bot API 9.5+
    ..Default::default()
};
app.set_bottom_button_params(BottomButton::Main, &params)?;
# Ok(())
# }
```

Convenience aliases exist for the main button (`set_main_button_text`,
`show_main_button`, `enable_main_button`, `set_main_button_callback`, …) and the
secondary button (`set_secondary_button_text`, `show_secondary_button`, …).

Back and Settings buttons are driven directly:

```rust,no_run
# use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.show_back_button()?;
let handle = app.set_back_button_callback(|| { /* navigate back */ })?;
app.remove_back_button_callback(handle)?;

app.show_settings_button()?;
let sh = app.set_settings_button_callback(|| { /* open settings */ })?;
app.remove_settings_button_callback(sh)?;
# Ok(())
# }
```

## Dialogs

```rust,no_run
# use telegram_webapp_sdk::webapp::TelegramWebApp;
# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.show_alert("Saved!")?;
let ok: bool = app.show_confirm("Delete this item?").await?;
let button_id: String = app.show_popup(&wasm_bindgen::JsValue::NULL).await?;
let code: String = app.show_scan_qr_popup("Point at a QR code").await?;
app.close_scan_qr_popup()?;
let _ = (ok, button_id, code);
# Ok(())
# }
```

## Navigation and links

`open_link` (with optional `OpenLinkOptions`), `open_telegram_link`,
`switch_inline_query`, `share_url`, `share_message` / `share_to_story`,
`request_chat`, `add_to_home_screen`, and `check_home_screen_status`.

## Theme and colors

`set_header_color`, `set_background_color`, `set_bottom_bar_color`,
`color_scheme()`, plus the free function
`telegram_webapp_sdk::api::theme::get_theme_params()` returning a parsed palette.

## Viewport and safe area

`viewport_height()`, `viewport_width()`, `viewport_stable_height()`,
`expand_viewport()`, and the `SafeAreaInset` accessors `safe_area_inset()` /
`content_safe_area_inset()`. Fullscreen and orientation live under
`request_fullscreen`, `exit_fullscreen`, `is_fullscreen`, `lock_orientation`,
and `unlock_orientation`.

## Storage

- **CloudStorage** — `api::cloud_storage::{get_item, set_item, remove_item, get_items, remove_items, get_keys}` (each returns a JS `Promise`).
- **DeviceStorage** — `api::device_storage::{set, get, remove, clear}` (`async`).
- **SecureStorage** — `api::secure_storage::{set, get, restore, remove, clear}` (`async`).

```rust,no_run
use telegram_webapp_sdk::api::device_storage::{get, set};
# async fn run() -> Result<(), wasm_bindgen::JsValue> {
set("theme", "dark").await?;
let value: Option<String> = get("theme").await?;
let _ = value;
# Ok(())
# }
```

## Sensors

Accelerometer, gyroscope, and device orientation each expose `start()`,
`stop()`, a getter (`get_acceleration()`, `get_angular_velocity()`,
`get_orientation()`), and lifecycle callbacks `on_started` / `on_changed` /
`on_stopped` / `on_failed`. LocationManager offers `init()`, `get_location()`,
`open_settings()`, `on_location_manager_updated`, and `on_location_requested`.

```rust,no_run
use telegram_webapp_sdk::api::accelerometer::{get_acceleration, start, stop};
start()?;
let reading = get_acceleration();
stop()?;
let _ = reading;
# Ok::<(), wasm_bindgen::JsValue>(())
```

## Biometry

`api::biometric::{init, is_biometric_available, request_access, authenticate, update_biometric_token, open_settings, is_inited, is_access_granted, device_id, …}`.

```rust,no_run
use telegram_webapp_sdk::api::biometric::{authenticate, init, is_biometric_available, request_access};
# fn run() -> Result<(), wasm_bindgen::JsValue> {
init()?;
if is_biometric_available()? {
    request_access("auth-key", Some("Unlock the vault"), None)?;
    authenticate("auth-key", None, None)?;
}
# Ok(())
# }
```

## Haptics

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

See also [Framework Integration](Framework-Integration-en) for reactive wrappers
over viewport, theme, and safe-area events.
