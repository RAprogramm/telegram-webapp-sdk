# Framework Integration

The crate ships optional integrations for [Leptos](https://leptos.dev) (feature
`leptos`) and [Yew](https://yew.rs) (feature `yew`). Both expose the same three
reactive hooks and the same three system-button components. The hooks seed their
state with the current values and re-render when Telegram fires
`viewportChanged`, `themeChanged`, `safeAreaChanged`, or
`contentSafeAreaChanged`. Subscriptions are cleaned up automatically on
unmount / scope disposal.

## Leptos

Provide the context once near the root, then read it with `use_context`:

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::{core::context::TelegramContext, leptos::provide_telegram_context};

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let ctx = use_context::<TelegramContext>().expect("context");
    view! { <span>{ ctx.init_data.auth_date }</span> }
}
```

### Reactive hooks

In Leptos the hooks return `ReadSignal<T>`, so you read them through `.get()`
inside a closure:

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::leptos::{use_safe_area, use_theme, use_viewport};

#[component]
fn Status() -> impl IntoView {
    let viewport = use_viewport(); // ReadSignal<ViewportState>
    let theme = use_theme();       // ReadSignal<ThemeState>
    let safe = use_safe_area();    // ReadSignal<SafeAreaState>
    view! {
        <div>
            { move || viewport.get().height }
            { move || theme.get().color_scheme.unwrap_or_default() }
            { move || safe.get().area.map(|i| i.top).unwrap_or(0.0) }
        </div>
    }
}
```

`ViewportState` carries `height`, `stable_height`, and `is_expanded`.
`ThemeState` carries `color_scheme: Option<String>` and a parsed `params`
palette. `SafeAreaState` carries `area` and `content` as `Option<SafeAreaInset>`.

### Components

`BottomButton` takes a `button` selector plus reactive `text` (and optional
`color` / `text_color` / `on_click`). `BackButton` and `SettingsButton` take a
reactive `visible` signal and an optional `on_click` closure. All three drive the
native Telegram buttons and clean up on unmount.

```rust,ignore
use leptos::prelude::*;
use telegram_webapp_sdk::{
    leptos::{provide_telegram_context, BackButton, BottomButton, SettingsButton},
    webapp::BottomButton as Btn,
};

#[component]
fn App() -> impl IntoView {
    provide_telegram_context().expect("context");
    let (text, _set_text) = signal("Send".to_owned());
    let visible = RwSignal::new(true);
    view! {
        <BottomButton button=Btn::Main text on_click=move || { /* submit */ } />
        <BackButton visible=visible on_click=move || { /* navigate back */ } />
        <SettingsButton visible=visible on_click=move || { /* open settings */ } />
    }
}
```

## Yew

Read the context with the `use_telegram_context` hook. It returns
`Result<TelegramContext, JsValue>` and reactively resolves once the context is
initialized:

```rust,ignore
use telegram_webapp_sdk::yew::use_telegram_context;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    match use_telegram_context() {
        Ok(ctx) => html! { <span>{ ctx.init_data.auth_date }</span> },
        Err(_) => html! { <div>{ "Loading Telegram context..." }</div> },
    }
}
```

### Reactive hooks

In Yew the hooks return the state value directly (not a signal), so you read
fields straight off it:

```rust,ignore
use telegram_webapp_sdk::yew::{use_safe_area, use_theme, use_viewport};
use yew::prelude::*;

#[function_component(Status)]
fn status() -> Html {
    let viewport = use_viewport(); // ViewportState
    let theme = use_theme();       // ThemeState
    let safe = use_safe_area();    // SafeAreaState
    html! {
        <div>
            { viewport.height }
            { theme.color_scheme.clone().unwrap_or_default() }
            { safe.area.map(|i| i.top).unwrap_or(0.0) }
        </div>
    }
}
```

### Components

Yew's components take plain props: `BottomButton` uses `text` / `color` /
`text_color` / `on_click` (it always drives the Main button), while `BackButton`
and `SettingsButton` take a `visible: bool` prop and an `on_click: Callback<()>`.

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

## See also

- [Quick Start](Quick-Start-en) — the vanilla (no-framework) flow
- [Mock & Testing](Testing-en) — render components against a mock environment
- [Examples](Examples-en) — a runnable Trunk demo
