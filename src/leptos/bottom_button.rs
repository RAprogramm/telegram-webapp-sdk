use std::{cell::RefCell, collections::HashMap};

use leptos::prelude::*;

use crate::{
    logger,
    webapp::{BottomButton as WebBottomButton, EventHandle, TelegramWebApp}
};

thread_local! {
    static BUTTON_HANDLES: RefCell<HashMap<WebBottomButton, EventHandle<dyn FnMut()>>> =
        RefCell::new(HashMap::new());
}

/// Leptos component that controls a Telegram bottom button.
///
/// The component shows the selected bottom button and keeps its text and
/// colors in sync with the provided reactive signals. An optional click
/// callback can be registered and is automatically removed when the component
/// unmounts.
///
/// # Examples
///
/// ```no_run
/// use leptos::prelude::*;
/// use telegram_webapp_sdk::{
///     leptos::{BottomButton, provide_telegram_context},
///     webapp::{BottomButton as Btn, TelegramWebApp}
/// };
///
/// #[component]
/// fn App() -> impl IntoView {
///     provide_telegram_context().expect("context");
///     let (text, _set_text) = signal("Send".to_owned());
///     view! {
///         <BottomButton
///             button=Btn::Main
///             text
///             on_click=move || {
///                 if let Some(app) = TelegramWebApp::instance() {
///                     let _ = app.send_data("clicked");
///                 }
///             }
///         />
///     }
/// }
/// ```
#[component]
pub fn BottomButton<F>(
    #[prop(into)] text: Signal<String>,
    #[prop(optional, into)] color: Option<Signal<String>>,
    #[prop(optional, into)] text_color: Option<Signal<String>>,
    #[prop(optional)] on_click: Option<F>,
    #[prop(default = WebBottomButton::Main)] button: WebBottomButton
) -> impl IntoView
where
    F: Fn() + Clone + 'static
{
    // Show button on mount.
    Effect::new(move |_| {
        if let Some(app) = TelegramWebApp::instance() {
            if let Err(err) = app.show_bottom_button(button) {
                logger::error(&format!("show_bottom_button failed: {err:?}"));
            }
        } else {
            logger::error("TelegramWebApp instance not available");
        }
    });

    // Update text when signal changes.
    Effect::new(move |_| {
        if let Some(app) = TelegramWebApp::instance()
            && let Err(err) = app.set_bottom_button_text(button, &text.get())
        {
            logger::error(&format!("set_bottom_button_text failed: {err:?}"));
        }
    });

    // Update button color.
    if let Some(color) = color {
        Effect::new(move |_| {
            if let Some(app) = TelegramWebApp::instance()
                && let Err(err) = app.set_bottom_button_color(button, &color.get())
            {
                logger::error(&format!("set_bottom_button_color failed: {err:?}"));
            }
        });
    }

    // Update text color.
    if let Some(text_color) = text_color {
        Effect::new(move |_| {
            if let Some(app) = TelegramWebApp::instance()
                && let Err(err) = app.set_bottom_button_text_color(button, &text_color.get())
            {
                logger::error(&format!("set_bottom_button_text_color failed: {err:?}"));
            }
        });
    }

    // Register click callback if provided and keep handle for cleanup.
    if let Some(cb) = on_click {
        if let Some(app) = TelegramWebApp::instance() {
            match app.set_bottom_button_callback(button, cb) {
                Ok(handle) => BUTTON_HANDLES.with(|handles| {
                    handles.borrow_mut().insert(button, handle);
                }),
                Err(err) => logger::error(&format!("set_bottom_button_callback failed: {err:?}"))
            }
        } else {
            logger::error("TelegramWebApp instance not available");
        }
    }

    // Cleanup: remove callback and hide button when component unmounts.
    on_cleanup(move || {
        if let Some(app) = TelegramWebApp::instance() {
            BUTTON_HANDLES.with(|handles| {
                if let Some(handle) = handles.borrow_mut().remove(&button) {
                    if let Err(err) = app.remove_bottom_button_callback(handle) {
                        logger::error(&format!("remove_bottom_button_callback failed: {err:?}"));
                    }
                }
            });
            if let Err(err) = app.hide_bottom_button(button) {
                logger::error(&format!("hide_bottom_button failed: {err:?}"));
            }
        } else {
            logger::error("TelegramWebApp instance not available");
        }
    });

    // Component renders no DOM nodes.
    View::new(())
}

#[cfg(all(test, feature = "leptos"))]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc
    };

    use js_sys::{Function, Object, Reflect};
    use leptos::prelude::*;
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::window;

    use super::BottomButton;
    use crate::webapp::BottomButton as WebBottomButton;

    #[allow(dead_code, clippy::type_complexity)]
    fn setup_webapp() -> (
        Rc<Cell<bool>>,
        Rc<Cell<bool>>,
        Rc<RefCell<Vec<String>>>,
        Rc<RefCell<Option<Function>>>
    ) {
        let win = window().unwrap();
        let tg = Object::new();
        let webapp = Object::new();
        let button = Object::new();

        let show_called = Rc::new(Cell::new(false));
        let show_cb = {
            let show_called = Rc::clone(&show_called);
            Closure::<dyn FnMut()>::new(move || {
                show_called.set(true);
            })
        };
        Reflect::set(&button, &"show".into(), show_cb.as_ref().unchecked_ref()).unwrap();
        show_cb.forget();

        let hide_called = Rc::new(Cell::new(false));
        let hide_cb = {
            let hide_called = Rc::clone(&hide_called);
            Closure::<dyn FnMut()>::new(move || {
                hide_called.set(true);
            })
        };
        Reflect::set(&button, &"hide".into(), hide_cb.as_ref().unchecked_ref()).unwrap();
        hide_cb.forget();

        let texts = Rc::new(RefCell::new(Vec::new()));
        let set_text_cb = {
            let texts = Rc::clone(&texts);
            Closure::<dyn FnMut(JsValue)>::new(move |val: JsValue| {
                if let Some(s) = val.as_string() {
                    texts.borrow_mut().push(s);
                }
            })
        };
        Reflect::set(
            &button,
            &"setText".into(),
            set_text_cb.as_ref().unchecked_ref()
        )
        .unwrap();
        set_text_cb.forget();

        let click_fn = Rc::new(RefCell::new(None));
        let on_click_cb = {
            let click_fn = Rc::clone(&click_fn);
            Closure::<dyn Fn(JsValue)>::new(move |f: JsValue| {
                *click_fn.borrow_mut() = f.dyn_into::<Function>().ok();
            })
        };
        Reflect::set(
            &button,
            &"onClick".into(),
            on_click_cb.as_ref().unchecked_ref()
        )
        .unwrap();
        on_click_cb.forget();

        let off_click_cb = {
            let click_fn = Rc::clone(&click_fn);
            Closure::<dyn FnMut(JsValue)>::new(move |_f: JsValue| {
                *click_fn.borrow_mut() = None;
            })
        };
        Reflect::set(
            &button,
            &"offClick".into(),
            off_click_cb.as_ref().unchecked_ref()
        )
        .unwrap();
        off_click_cb.forget();

        Reflect::set(&webapp, &"MainButton".into(), &button).unwrap();
        Reflect::set(&tg, &"WebApp".into(), &webapp).unwrap();
        Reflect::set(&win, &"Telegram".into(), &tg).unwrap();

        (show_called, hide_called, texts, click_fn)
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn bottom_button_updates_and_cleans_up() {
        let (show_called, hide_called, texts, click_fn) = setup_webapp();

        let owner = Owner::new();
        owner.set();
        let (text, set_text) = signal("Start".to_owned());
        let clicked = Rc::new(Cell::new(false));
        let clicked_clone = Rc::clone(&clicked);
        let _view = view! {
            <BottomButton
                button=WebBottomButton::Main
                text
                on_click=move || clicked_clone.set(true)
            />
        };
        set_text.set("Next".to_owned());
        if let Some(func) = click_fn.borrow().as_ref() {
            let _ = func.call0(&JsValue::NULL);
        }
        assert!(clicked.get());
        drop(owner);

        assert!(show_called.get());
        assert!(hide_called.get());
        assert!(click_fn.borrow().is_none());
        let stored = texts.borrow();
        assert_eq!(stored.as_slice(), ["Start", "Next"]);
    }
}
