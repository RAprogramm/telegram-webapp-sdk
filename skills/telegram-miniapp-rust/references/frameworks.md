# Framework bindings (Yew and Leptos)

Both features expose the same surface: `BackButton`, `BottomButton`,
`SettingsButton` components plus `use_theme`, `use_viewport`, `use_safe_area`
hooks. The components drive the real Telegram buttons — they never render
HTML buttons.

## Yew (`features = ["yew"]`)

```rust
use telegram_webapp_sdk::yew::BottomButton;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let on_click = Callback::from(|_| {
        // submit logic
    });
    html! { <BottomButton text="Confirm" color="#2AABEE" text_color="#FFFFFF" {on_click} /> }
}
```

Theme and viewport arrive as reactive state:

```rust
use telegram_webapp_sdk::yew::{use_theme, use_viewport};

let theme = use_theme();     // updates on themeChanged
let viewport = use_viewport(); // expands/contracts with the client
```

## Leptos (`features = ["leptos"]`)

Provide the initialized context once at the app root, then read it anywhere:

```rust
use leptos::prelude::*;
use telegram_webapp_sdk::{
    core::context::TelegramContext,
    leptos::{provide_telegram_context, use_telegram_context, BottomButton}
};

#[component]
pub fn App() -> impl IntoView {
    provide_telegram_context().expect("init_sdk() must run before mount");
    view! { <BottomButton button=telegram_webapp_sdk::webapp::BottomButton::Main text="Confirm" /> }
}

#[component]
fn WhoAmI() -> impl IntoView {
    let ctx = use_telegram_context().ok()?;
    move || format!("id: {:?}", ctx.init_data.user.as_ref().map(|u| u.id))
}
```

## Rules

- Effects in Leptos components spawn on the local executor: in tests call
  `any_spawner::Executor::init_wasm_bindgen()` before rendering; in the app
  Trunk's start path already provides it.
- Components hide/remove the underlying Telegram button on unmount — do not
  call `hide_bottom_button` manually alongside them.
- Never mount framework test renders into `document.body`; append a `div`
  and render into it (the wasm-bindgen-test harness lives in `body`).
