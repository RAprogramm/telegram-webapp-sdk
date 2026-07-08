# Home

`telegram-webapp-sdk` is a type-safe, ergonomic Rust/WASM wrapper around the
[Telegram Web Apps](https://core.telegram.org/bots/webapps) JavaScript API. It
lets you build Telegram Mini Apps entirely in Rust — with vanilla WebAssembly,
or through first-class [Yew](https://yew.rs) and [Leptos](https://leptos.dev)
integrations.

## API coverage

The crate tracks the **Telegram WebApp API version 9.6**, which is the latest
Mini App surface exposed by `telegram-web-app.js`. Bot API has since advanced to
10.1, but versions 9.7–10.1 introduced no new WebApp methods, fields, or events,
so the covered surface is complete. Bot API 9.5 added `icon_custom_emoji_id` for
bottom buttons; 9.6 added `WebApp.requestChat` and the `requestedChatSent` /
`requestedChatFailed` events — both are implemented.

The covered surface includes:

- Buttons: Main, Secondary, Back, and Settings buttons
- Dialogs: alert, confirm, popup, and QR-code scanner
- Navigation, links, sharing, and invoices
- Theme, colors, viewport, and safe-area insets
- Storage: CloudStorage, DeviceStorage, and SecureStorage
- Sensors: accelerometer, gyroscope, device orientation, and LocationManager
- Biometric authentication and haptic feedback

For the full method-by-method checklist, see
[`WEBAPP_API.md`](https://github.com/RAprogramm/telegram-webapp-sdk/blob/main/WEBAPP_API.md).

## Feature flags

The crate is `default = []` — enable only what you need.

| Feature  | What it enables |
|----------|-----------------|
| `macros` | `telegram_app!`, `telegram_page!`, and `telegram_router!` for boilerplate-free apps and routing |
| `yew`    | `use_telegram_context`, reactive `use_viewport` / `use_theme` / `use_safe_area` hooks, and `BottomButton` / `BackButton` / `SettingsButton` components |
| `leptos` | `provide_telegram_context`, the same reactive `use_*` hooks, and matching Leptos components |
| `mock`   | A configurable mock `Telegram.WebApp` for local development and testing |
| `full`   | Aggregates `macros`, `yew`, `leptos`, and `mock` |

```toml
telegram-webapp-sdk = { version = "0.11", features = ["leptos", "mock"] }
```

## Documentation index

- [Installation](Installation-en) — dependency line, feature flags, MSRV, and the wasm32 target
- [Quick Start](Quick-Start-en) — obtain the WebApp instance, `ready()`/`expand()`, read init data, and wire up a MainButton
- [WebApp API](WebApp-API-en) — a tour of the covered surface grouped by area, plus the `*_with_callback` vs `async` pattern
- [Framework Integration](Framework-Integration-en) — Leptos and Yew hooks and components
- [Mock & Testing](Testing-en) — the `mock` feature and `wasm_bindgen_test`
- [Examples](Examples-en) — the demo app, vanilla example, bot, and full-stack integration

## License

`telegram-webapp-sdk` is licensed under the MIT license.
