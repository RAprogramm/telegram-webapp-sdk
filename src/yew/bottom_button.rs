// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use yew::prelude::*;

use crate::webapp::{BottomButton as TgBottomButton, TelegramWebApp};

/// Yew component that configures the primary Telegram bottom button.
///
/// The button is shown when the component is mounted and hidden on drop.
/// Text, colors and callback can be customized through [`BottomButtonProps`].
///
/// # Examples
///
/// ```no_run
/// use telegram_webapp_sdk::yew::BottomButton;
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// fn app() -> Html {
///     let on_click = Callback::from(|_| {});
///     html! { <BottomButton text="OK" color="#000" text_color="#fff" {on_click} /> }
/// }
/// ```
#[function_component(BottomButton)]
pub fn bottom_button(props: &BottomButtonProps) -> Html {
    use_effect_with(props.clone(), |props| {
        if let Some(app) = TelegramWebApp::instance() {
            if let Some(color) = props.color.as_ref() {
                let _ = app.set_bottom_button_color(TgBottomButton::Main, color.as_ref());
            }
            if let Some(text_color) = props.text_color.as_ref() {
                let _ =
                    app.set_bottom_button_text_color(TgBottomButton::Main, text_color.as_ref());
            }
            let _ = app.set_bottom_button_text(TgBottomButton::Main, props.text.as_ref());

            let cb = props.on_click.clone();
            let handle = app
                .set_bottom_button_callback(TgBottomButton::Main, move || cb.emit(()))
                .ok();
            let _ = app.show_bottom_button(TgBottomButton::Main);

            return Box::new(move || {
                if let Some(h) = handle
                    && let Some(app) = TelegramWebApp::instance()
                {
                    let _ = app.remove_bottom_button_callback(h);
                    let _ = app.hide_bottom_button(TgBottomButton::Main);
                }
            }) as Box<dyn FnOnce()>;
        }

        Box::new(|| {}) as Box<dyn FnOnce()>
    });

    Html::default()
}

/// Properties for [`BottomButton`].
#[derive(Properties, PartialEq, Clone)]
pub struct BottomButtonProps {
    /// Button text.
    pub text:       AttrValue,
    /// Background color in hex format.
    #[prop_or_default]
    pub color:      Option<AttrValue>,
    /// Text color in hex format.
    #[prop_or_default]
    pub text_color: Option<AttrValue>,
    /// Callback triggered on button click.
    pub on_click:   Callback<()>
}

#[cfg(all(test, feature = "yew"))]
mod tests {
    use std::rc::Rc;

    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().expect("window should be available");
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn renders_and_registers_callback() {
        let webapp = setup_webapp();
        let main = Object::new();

        let show = Function::new_with_args("", "this.show_called = true;");
        let set_text = Function::new_with_args("t", "this.text = t;");
        let set_color = Function::new_with_args("c", "this.color = c;");
        let set_text_color = Function::new_with_args("c", "this.text_color = c;");
        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "this.cb = undefined;");
        let _ = Reflect::set(&main, &"show".into(), &show);
        let _ = Reflect::set(&main, &"setText".into(), &set_text);
        let _ = Reflect::set(&main, &"setColor".into(), &set_color);
        let _ = Reflect::set(&main, &"setTextColor".into(), &set_text_color);
        let _ = Reflect::set(&main, &"onClick".into(), &on_click);
        let _ = Reflect::set(&main, &"offClick".into(), &off_click);
        let _ = Reflect::set(&webapp, &"MainButton".into(), &main);

        let clicked = Rc::new(std::cell::Cell::new(false));
        let clicked_clone = Rc::clone(&clicked);
        let props = BottomButtonProps {
            text:       AttrValue::from("Press"),
            color:      Some(AttrValue::from("#000000")),
            text_color: Some(AttrValue::from("#ffffff")),
            on_click:   Callback::from(move |_| clicked_clone.set(true))
        };
        let document = window().unwrap().document().unwrap();
        yew::Renderer::<BottomButton>::with_root_and_props(document.body().unwrap().into(), props)
            .render();

        let show_called = Reflect::get(&main, &"show_called".into())
            .unwrap_or(JsValue::FALSE)
            .as_bool()
            .unwrap_or(false);
        assert!(show_called);

        let text = Reflect::get(&main, &"text".into())
            .unwrap_or(JsValue::NULL)
            .as_string()
            .unwrap();
        assert_eq!(text, "Press");

        let cb = Reflect::get(&main, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb.call0(&JsValue::NULL);
        assert!(clicked.get());
    }
}
