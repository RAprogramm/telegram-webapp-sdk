# telegram-webapp-sdk

A Rust library for building Telegram Web Apps with an ergonomic and safe API.

## Features

- Core utilities to interact with the `Telegram.WebApp` interface.
- Optional framework integrations activated via Cargo features:
  - `yew` &mdash; provides a Yew hook `use_telegram_context` for retrieving the global `TelegramContext`.
  - `leptos` &mdash; offers `provide_telegram_context` to inject the `TelegramContext` into the Leptos reactive system.

Enable features in `Cargo.toml`:

```toml
telegram-webapp-sdk = { version = "0.1", features = ["yew"] }
```

## Examples

### Yew

```rust,no_run
use yew::prelude::*;
use telegram_webapp_sdk::yew::use_telegram_context;

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
