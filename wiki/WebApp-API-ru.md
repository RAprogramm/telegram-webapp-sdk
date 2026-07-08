# WebApp API

Почти все возможности доступны через хендл `TelegramWebApp`
(`instance()` / `try_instance()`), плюс несколько модулей со свободными
функциями в `telegram_webapp_sdk::api::*` для сенсоров, хранилищ, тактильной
отдачи и темы. Эта страница группирует поверхность по разделам. Исчерпывающий
чек-лист — в
[`WEBAPP_API.md`](https://github.com/RAprogramm/telegram-webapp-sdk/blob/main/WEBAPP_API.md).

## Паттерн `*_with_callback` против `async`

У каждого одноразового колбэка Telegram есть **два** Rust-сиблинга:

- `foo_with_callback(..., F)` — синхронная регистрация; ваше замыкание
  вызывается, когда Telegram присылает результат. Используйте, когда нельзя
  `.await` (например, внутри не-async замыкания).
- `async fn foo(...)` — возвращает естественный Rust-тип через `.await`.
  Предпочтительно в продакшене.

```rust,no_run
use telegram_webapp_sdk::webapp::TelegramWebApp;

# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

let confirmed: bool = app.show_confirm("Отправить заказ?").await?;
let scanned: String = app.show_scan_qr_popup("Сканируйте QR-код").await?;
let granted: bool = app.request_write_access().await?;
let _ = (confirmed, scanned, granted);
# Ok(())
# }
```

То же справедливо для `share_message`, `request_chat`,
`check_home_screen_status`, `set_emoji_status`, `request_emoji_status_access`,
`open_invoice`, `download_file`, `read_text_from_clipboard`, `show_popup` и
`invoke_custom_method`.

## Кнопки

Нижние кнопки Main и Secondary используют enum-селектор `BottomButton`
(`BottomButton::Main` / `BottomButton::Secondary`):

```rust,no_run
use telegram_webapp_sdk::webapp::{BottomButton, BottomButtonParams, TelegramWebApp};

# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;

app.set_bottom_button_text(BottomButton::Main, "Отправить")?;
app.show_bottom_button(BottomButton::Main)?;

// Атомарное обновление через setParams:
let params = BottomButtonParams {
    text: Some("Отправить"),
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

Есть удобные алиасы для главной кнопки (`set_main_button_text`,
`show_main_button`, `enable_main_button`, `set_main_button_callback`, …) и для
вторичной (`set_secondary_button_text`, `show_secondary_button`, …).

Кнопки Back и Settings управляются напрямую:

```rust,no_run
# use telegram_webapp_sdk::webapp::TelegramWebApp;
# fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.show_back_button()?;
let handle = app.set_back_button_callback(|| { /* назад */ })?;
app.remove_back_button_callback(handle)?;

app.show_settings_button()?;
let sh = app.set_settings_button_callback(|| { /* настройки */ })?;
app.remove_settings_button_callback(sh)?;
# Ok(())
# }
```

## Диалоги

```rust,no_run
# use telegram_webapp_sdk::webapp::TelegramWebApp;
# async fn run() -> Result<(), wasm_bindgen::JsValue> {
let app = TelegramWebApp::try_instance()?;
app.show_alert("Сохранено!")?;
let ok: bool = app.show_confirm("Удалить элемент?").await?;
let button_id: String = app.show_popup(&wasm_bindgen::JsValue::NULL).await?;
let code: String = app.show_scan_qr_popup("Наведите на QR-код").await?;
app.close_scan_qr_popup()?;
let _ = (ok, button_id, code);
# Ok(())
# }
```

## Навигация и ссылки

`open_link` (с опциональным `OpenLinkOptions`), `open_telegram_link`,
`switch_inline_query`, `share_url`, `share_message` / `share_to_story`,
`request_chat`, `add_to_home_screen` и `check_home_screen_status`.

## Тема и цвета

`set_header_color`, `set_background_color`, `set_bottom_bar_color`,
`color_scheme()`, а также свободная функция
`telegram_webapp_sdk::api::theme::get_theme_params()`, возвращающая разобранную
палитру.

## Вьюпорт и safe-area

`viewport_height()`, `viewport_width()`, `viewport_stable_height()`,
`expand_viewport()` и аксессоры `SafeAreaInset`: `safe_area_inset()` /
`content_safe_area_inset()`. Полноэкранный режим и ориентация:
`request_fullscreen`, `exit_fullscreen`, `is_fullscreen`, `lock_orientation` и
`unlock_orientation`.

## Хранилища

- **CloudStorage** — `api::cloud_storage::{get_item, set_item, remove_item, get_items, remove_items, get_keys}` (каждая возвращает JS `Promise`).
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

## Сенсоры

Акселерометр, гироскоп и ориентация устройства предоставляют `start()`,
`stop()`, геттер (`get_acceleration()`, `get_angular_velocity()`,
`get_orientation()`) и колбэки жизненного цикла `on_started` / `on_changed` /
`on_stopped` / `on_failed`. LocationManager: `init()`, `get_location()`,
`open_settings()`, `on_location_manager_updated` и `on_location_requested`.

```rust,no_run
use telegram_webapp_sdk::api::accelerometer::{get_acceleration, start, stop};
start()?;
let reading = get_acceleration();
stop()?;
let _ = reading;
# Ok::<(), wasm_bindgen::JsValue>(())
```

## Биометрия

`api::biometric::{init, is_biometric_available, request_access, authenticate, update_biometric_token, open_settings, is_inited, is_access_granted, device_id, …}`.

```rust,no_run
use telegram_webapp_sdk::api::biometric::{authenticate, init, is_biometric_available, request_access};
# fn run() -> Result<(), wasm_bindgen::JsValue> {
init()?;
if is_biometric_available()? {
    request_access("auth-key", Some("Разблокировать хранилище"), None)?;
    authenticate("auth-key", None, None)?;
}
# Ok(())
# }
```

## Тактильная отдача

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

См. также [Интеграция с фреймворками](Интеграция) — реактивные обёртки над
событиями вьюпорта, темы и safe-area.
