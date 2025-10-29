// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::Object;

// Module declarations
mod buttons;
mod core;
mod dialogs;
mod events;
mod lifecycle;
mod navigation;
mod permissions;
mod theme;
pub mod types;
mod viewport;

// Re-export public types
pub use types::{
    BackgroundEvent, BottomButton, BottomButtonParams, EventHandle, OpenLinkOptions,
    SafeAreaInset, SecondaryButtonParams, SecondaryButtonPosition
};

/// Safe wrapper around `window.Telegram.WebApp`
#[derive(Clone)]
pub struct TelegramWebApp {
    pub(super) inner: Object
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc
    };

    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;
    use crate::core::types::download_file_params::DownloadFileParams;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_keyboard_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"hideKeyboard".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.hide_keyboard().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_main_button_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &main_button,
            &"hide".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.hide_bottom_button(BottomButton::Main).unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_secondary_button_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &secondary_button,
            &"hide".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.hide_bottom_button(BottomButton::Secondary).unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_button_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_color(BottomButton::Main, "#00ff00")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_secondary_button_color_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &secondary_button,
            &"setColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_color(BottomButton::Secondary, "#00ff00")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_button_text_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setTextColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_text_color(BottomButton::Main, "#112233")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_secondary_button_text_color_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &secondary_button,
            &"setTextColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_text_color(BottomButton::Secondary, "#112233")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn enable_bottom_button_calls_js() {
        let webapp = setup_webapp();
        let button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let enable_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &button,
            &"enable".into(),
            enable_cb.as_ref().unchecked_ref()
        );
        enable_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        app.enable_bottom_button(BottomButton::Main).unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_bottom_button_progress_passes_flag() {
        let webapp = setup_webapp();
        let button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |arg: JsValue| {
            *rc_clone.borrow_mut() = arg.as_bool();
        });
        let _ = Reflect::set(&button, &"showProgress".into(), cb.as_ref().unchecked_ref());
        cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        app.show_bottom_button_progress(BottomButton::Main, true)
            .unwrap();
        assert_eq!(*received.borrow(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_button_params_serializes() {
        let webapp = setup_webapp();
        let button = Object::new();
        let received = Rc::new(RefCell::new(Object::new()));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| {
            let obj = value.dyn_into::<Object>().expect("object");
            rc_clone.replace(obj);
        });
        let _ = Reflect::set(&button, &"setParams".into(), cb.as_ref().unchecked_ref());
        cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        let params = BottomButtonParams {
            text:             Some("Send"),
            color:            Some("#ffffff"),
            text_color:       Some("#000000"),
            is_active:        Some(true),
            is_visible:       Some(true),
            has_shine_effect: Some(false)
        };
        app.set_bottom_button_params(BottomButton::Main, &params)
            .unwrap();

        let stored = received.borrow();
        assert_eq!(
            Reflect::get(&stored, &"text".into()).unwrap().as_string(),
            Some("Send".to_string())
        );
        assert_eq!(
            Reflect::get(&stored, &"color".into()).unwrap().as_string(),
            Some("#ffffff".to_string())
        );
        assert_eq!(
            Reflect::get(&stored, &"text_color".into())
                .unwrap()
                .as_string(),
            Some("#000000".to_string())
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_secondary_button_params_serializes_position() {
        let webapp = setup_webapp();
        let button = Object::new();
        let received = Rc::new(RefCell::new(Object::new()));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| {
            let obj = value.dyn_into::<Object>().expect("object");
            rc_clone.replace(obj);
        });
        let _ = Reflect::set(&button, &"setParams".into(), cb.as_ref().unchecked_ref());
        cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        let params = SecondaryButtonParams {
            common:   BottomButtonParams {
                text: Some("Next"),
                ..Default::default()
            },
            position: Some(SecondaryButtonPosition::Left)
        };
        app.set_secondary_button_params(&params).unwrap();

        let stored = received.borrow();
        assert_eq!(
            Reflect::get(&stored, &"text".into()).unwrap().as_string(),
            Some("Next".to_string())
        );
        assert_eq!(
            Reflect::get(&stored, &"position".into())
                .unwrap()
                .as_string(),
            Some("left".to_string())
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn bottom_button_getters_return_values() {
        let webapp = setup_webapp();
        let button = Object::new();
        let _ = Reflect::set(&button, &"text".into(), &"Label".into());
        let _ = Reflect::set(&button, &"textColor".into(), &"#111111".into());
        let _ = Reflect::set(&button, &"color".into(), &"#222222".into());
        let _ = Reflect::set(&button, &"isVisible".into(), &JsValue::TRUE);
        let _ = Reflect::set(&button, &"isActive".into(), &JsValue::TRUE);
        let _ = Reflect::set(&button, &"isProgressVisible".into(), &JsValue::FALSE);
        let _ = Reflect::set(&button, &"hasShineEffect".into(), &JsValue::TRUE);

        let _ = Reflect::set(&webapp, &"MainButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        assert_eq!(
            app.bottom_button_text(BottomButton::Main),
            Some("Label".into())
        );
        assert_eq!(
            app.bottom_button_text_color(BottomButton::Main),
            Some("#111111".into())
        );
        assert_eq!(
            app.bottom_button_color(BottomButton::Main),
            Some("#222222".into())
        );
        assert!(app.is_bottom_button_visible(BottomButton::Main));
        assert!(app.is_bottom_button_active(BottomButton::Main));
        assert!(!app.is_bottom_button_progress_visible(BottomButton::Main));
        assert!(app.bottom_button_has_shine_effect(BottomButton::Main));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn secondary_button_position_is_parsed() {
        let webapp = setup_webapp();
        let button = Object::new();
        let _ = Reflect::set(&button, &"position".into(), &"right".into());
        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &button);

        let app = TelegramWebApp::instance().unwrap();
        assert_eq!(
            app.secondary_button_position(),
            Some(SecondaryButtonPosition::Right)
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_header_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setHeaderColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_header_color("#abcdef").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#abcdef"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_background_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setBackgroundColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_background_color("#123456").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#123456"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_bar_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setBottomBarColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_bar_color("#654321").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#654321"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_dimensions() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportWidth".into(), &JsValue::from_f64(320.0));
        let _ = Reflect::set(
            &webapp,
            &"viewportStableHeight".into(),
            &JsValue::from_f64(480.0)
        );
        let app = TelegramWebApp::instance().unwrap();
        assert_eq!(app.viewport_width(), Some(320.0));
        assert_eq!(app.viewport_stable_height(), Some(480.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn version_check_invokes_js() {
        let webapp = setup_webapp();
        let cb = Function::new_with_args("v", "return v === '9.0';");
        let _ = Reflect::set(&webapp, &"isVersionAtLeast".into(), &cb);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_version_at_least("9.0").unwrap());
        assert!(!app.is_version_at_least("9.1").unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn safe_area_insets_are_parsed() {
        let webapp = setup_webapp();
        let safe_area = Object::new();
        let _ = Reflect::set(&safe_area, &"top".into(), &JsValue::from_f64(1.0));
        let _ = Reflect::set(&safe_area, &"bottom".into(), &JsValue::from_f64(2.0));
        let _ = Reflect::set(&safe_area, &"left".into(), &JsValue::from_f64(3.0));
        let _ = Reflect::set(&safe_area, &"right".into(), &JsValue::from_f64(4.0));
        let _ = Reflect::set(&webapp, &"safeAreaInset".into(), &safe_area);

        let content_safe = Object::new();
        let _ = Reflect::set(&content_safe, &"top".into(), &JsValue::from_f64(5.0));
        let _ = Reflect::set(&content_safe, &"bottom".into(), &JsValue::from_f64(6.0));
        let _ = Reflect::set(&content_safe, &"left".into(), &JsValue::from_f64(7.0));
        let _ = Reflect::set(&content_safe, &"right".into(), &JsValue::from_f64(8.0));
        let _ = Reflect::set(&webapp, &"contentSafeAreaInset".into(), &content_safe);

        let app = TelegramWebApp::instance().unwrap();
        let inset = app.safe_area_inset().expect("safe area");
        assert_eq!(inset.top, 1.0);
        assert_eq!(inset.bottom, 2.0);
        assert_eq!(inset.left, 3.0);
        assert_eq!(inset.right, 4.0);

        let content = app.content_safe_area_inset().expect("content area");
        assert_eq!(content.top, 5.0);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn activity_flags_are_reported() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"isActive".into(), &JsValue::TRUE);
        let _ = Reflect::set(&webapp, &"isFullscreen".into(), &JsValue::TRUE);
        let _ = Reflect::set(&webapp, &"isOrientationLocked".into(), &JsValue::FALSE);
        let _ = Reflect::set(&webapp, &"isVerticalSwipesEnabled".into(), &JsValue::TRUE);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_active());
        assert!(app.is_fullscreen());
        assert!(!app.is_orientation_locked());
        assert!(app.is_vertical_swipes_enabled());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn back_button_visibility_and_callback() {
        let webapp = setup_webapp();
        let back_button = Object::new();
        let _ = Reflect::set(&webapp, &"BackButton".into(), &back_button);
        let _ = Reflect::set(&back_button, &"isVisible".into(), &JsValue::TRUE);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&back_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&back_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_back_button_visible());
        let handle = app
            .set_back_button_callback(move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&back_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_back_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn bottom_button_callback_register_and_remove() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&main_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&main_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .set_bottom_button_callback(BottomButton::Main, move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&main_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_bottom_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn secondary_button_callback_register_and_remove() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&secondary_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&secondary_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .set_bottom_button_callback(BottomButton::Secondary, move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&secondary_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&secondary_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_bottom_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&secondary_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn on_event_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_event("test", |_: JsValue| {}).unwrap();
        assert!(Reflect::has(&webapp, &"test".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"test".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn background_event_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .on_background_event(BackgroundEvent::MainButtonClicked, |_| {})
            .unwrap();
        assert!(Reflect::has(&webapp, &"mainButtonClicked".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"mainButtonClicked".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn background_event_delivers_data() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);

        let app = TelegramWebApp::instance().unwrap();
        let received = Rc::new(RefCell::new(String::new()));
        let received_clone = Rc::clone(&received);
        let _handle = app
            .on_background_event(BackgroundEvent::InvoiceClosed, move |v| {
                *received_clone.borrow_mut() = v.as_string().unwrap_or_default();
            })
            .unwrap();

        let cb = Reflect::get(&webapp, &"invoiceClosed".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb.call1(&JsValue::NULL, &JsValue::from_str("paid"));
        assert_eq!(received.borrow().as_str(), "paid");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn theme_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_theme_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"themeChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"themeChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn safe_area_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_safe_area_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"safeAreaChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"safeAreaChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn content_safe_area_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_content_safe_area_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"contentSafeAreaChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"contentSafeAreaChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_viewport_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn clipboard_text_received_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_clipboard_text_received(|_| {}).unwrap();
        assert!(Reflect::has(&webapp, &"clipboardTextReceived".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"clipboardTextReceived".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_link_and_telegram_link() {
        let webapp = setup_webapp();
        let open_link = Function::new_with_args("url", "this.open_link = url;");
        let open_tg_link = Function::new_with_args("url", "this.open_tg_link = url;");
        let _ = Reflect::set(&webapp, &"openLink".into(), &open_link);
        let _ = Reflect::set(&webapp, &"openTelegramLink".into(), &open_tg_link);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        app.open_link(url, None).unwrap();
        app.open_telegram_link(url).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"open_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
        assert_eq!(
            Reflect::get(&webapp, &"open_tg_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn invoice_closed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_invoice_closed(|_| {}).unwrap();
        assert!(Reflect::has(&webapp, &"invoiceClosed".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"invoiceClosed".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn invoice_closed_invokes_callback() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this.cb = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);
        app.on_invoice_closed(move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        let cb = Reflect::get(&webapp, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        cb.call1(&webapp, &"paid".into()).unwrap();
        assert_eq!(status.borrow().as_str(), "paid");
        cb.call1(&webapp, &"failed".into()).unwrap();
        assert_eq!(status.borrow().as_str(), "failed");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_invoice_invokes_callback() {
        let webapp = setup_webapp();
        let open_invoice = Function::new_with_args("url, cb", "cb('paid');");
        let _ = Reflect::set(&webapp, &"openInvoice".into(), &open_invoice);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);

        app.open_invoice("https://invoice", move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        assert_eq!(status.borrow().as_str(), "paid");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn switch_inline_query_calls_js() {
        let webapp = setup_webapp();
        let switch_inline =
            Function::new_with_args("query, types", "this.query = query; this.types = types;");
        let _ = Reflect::set(&webapp, &"switchInlineQuery".into(), &switch_inline);

        let app = TelegramWebApp::instance().unwrap();
        let types = JsValue::from_str("users");
        app.switch_inline_query("search", Some(&types)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"query".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("search"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"types".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("users"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_message_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args("id, cb", "this.shared_id = id; cb(true);");
        let _ = Reflect::set(&webapp, &"shareMessage".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let sent = Rc::new(Cell::new(false));
        let sent_clone = Rc::clone(&sent);

        app.share_message("123", move |s| {
            sent_clone.set(s);
        })
        .unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"shared_id".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("123"),
        );
        assert!(sent.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_to_story_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args(
            "url, params",
            "this.story_url = url; this.story_params = params;"
        );
        let _ = Reflect::set(&webapp, &"shareToStory".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com/media";
        let params = Object::new();
        let _ = Reflect::set(&params, &"text".into(), &"hi".into());
        app.share_to_story(url, Some(&params.into())).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"story_url".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url),
        );
        let stored = Reflect::get(&webapp, &"story_params".into()).unwrap();
        assert_eq!(
            Reflect::get(&stored, &"text".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("hi"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_url_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args(
            "url, text",
            "this.shared_url = url; this.shared_text = text;"
        );
        let _ = Reflect::set(&webapp, &"shareURL".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        let text = "check";
        app.share_url(url, Some(text)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"shared_url".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url),
        );
        assert_eq!(
            Reflect::get(&webapp, &"shared_text".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(text),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn join_voice_chat_calls_js() {
        let webapp = setup_webapp();
        let join = Function::new_with_args(
            "id, hash",
            "this.voice_chat_id = id; this.voice_chat_hash = hash;"
        );
        let _ = Reflect::set(&webapp, &"joinVoiceChat".into(), &join);

        let app = TelegramWebApp::instance().unwrap();
        app.join_voice_chat("123", Some("hash")).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_id".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("123"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_hash".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("hash"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn add_to_home_screen_calls_js() {
        let webapp = setup_webapp();
        let add = Function::new_with_args("", "this.called = true; return true;");
        let _ = Reflect::set(&webapp, &"addToHomeScreen".into(), &add);

        let app = TelegramWebApp::instance().unwrap();
        let shown = app.add_to_home_screen().unwrap();
        assert!(shown);
        let called = Reflect::get(&webapp, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(called);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_fullscreen_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"requestFullscreen".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.request_fullscreen().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn exit_fullscreen_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"exitFullscreen".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.exit_fullscreen().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn check_home_screen_status_invokes_callback() {
        let webapp = setup_webapp();
        let check = Function::new_with_args("cb", "cb('added');");
        let _ = Reflect::set(&webapp, &"checkHomeScreenStatus".into(), &check);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);

        app.check_home_screen_status(move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        assert_eq!(status.borrow().as_str(), "added");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn lock_orientation_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"lockOrientation".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.lock_orientation("portrait").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("portrait"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn unlock_orientation_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"unlockOrientation".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.unlock_orientation().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn enable_vertical_swipes_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"enableVerticalSwipes".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.enable_vertical_swipes().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn disable_vertical_swipes_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"disableVerticalSwipes".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.disable_vertical_swipes().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_write_access_invokes_callback() {
        let webapp = setup_webapp();
        let request = Function::new_with_args("cb", "cb(true);");
        let _ = Reflect::set(&webapp, &"requestWriteAccess".into(), &request);

        let app = TelegramWebApp::instance().unwrap();
        let granted = Rc::new(Cell::new(false));
        let granted_clone = Rc::clone(&granted);

        let res = app.request_write_access(move |g| {
            granted_clone.set(g);
        });
        assert!(res.is_ok());

        assert!(granted.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn download_file_invokes_callback() {
        let webapp = setup_webapp();
        let received_url = Rc::new(RefCell::new(String::new()));
        let received_name = Rc::new(RefCell::new(String::new()));
        let url_clone = Rc::clone(&received_url);
        let name_clone = Rc::clone(&received_name);

        let download = Closure::<dyn FnMut(JsValue, JsValue)>::new(move |params, cb: JsValue| {
            let url = Reflect::get(&params, &"url".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            let name = Reflect::get(&params, &"file_name".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            *url_clone.borrow_mut() = url;
            *name_clone.borrow_mut() = name;
            let func = cb.dyn_ref::<Function>().unwrap();
            let _ = func.call1(&JsValue::NULL, &JsValue::from_str("id"));
        });
        let _ = Reflect::set(
            &webapp,
            &"downloadFile".into(),
            download.as_ref().unchecked_ref()
        );
        download.forget();

        let app = TelegramWebApp::instance().unwrap();
        let result = Rc::new(RefCell::new(String::new()));
        let result_clone = Rc::clone(&result);
        let params = DownloadFileParams {
            url:       "https://example.com/data.bin",
            file_name: Some("data.bin"),
            mime_type: None
        };
        app.download_file(params, move |id| {
            *result_clone.borrow_mut() = id;
        })
        .unwrap();

        assert_eq!(
            received_url.borrow().as_str(),
            "https://example.com/data.bin"
        );
        assert_eq!(received_name.borrow().as_str(), "data.bin");
        assert_eq!(result.borrow().as_str(), "id");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_write_access_returns_error_when_missing() {
        let _webapp = setup_webapp();
        let app = TelegramWebApp::instance().unwrap();
        let res = app.request_write_access(|_| {});
        assert!(res.is_err());
    }
    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_emoji_status_access_invokes_callback() {
        let webapp = setup_webapp();
        let request = Function::new_with_args("cb", "cb(false);");
        let _ = Reflect::set(&webapp, &"requestEmojiStatusAccess".into(), &request);

        let app = TelegramWebApp::instance().unwrap();
        let granted = Rc::new(Cell::new(true));
        let granted_clone = Rc::clone(&granted);

        app.request_emoji_status_access(move |g| {
            granted_clone.set(g);
        })
        .unwrap();

        assert!(!granted.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_emoji_status_invokes_callback() {
        let webapp = setup_webapp();
        let set_status = Function::new_with_args("status, cb", "this.st = status; cb(true);");
        let _ = Reflect::set(&webapp, &"setEmojiStatus".into(), &set_status);

        let status = Object::new();
        let _ = Reflect::set(
            &status,
            &"custom_emoji_id".into(),
            &JsValue::from_str("321")
        );

        let app = TelegramWebApp::instance().unwrap();
        let success = Rc::new(Cell::new(false));
        let success_clone = Rc::clone(&success);

        app.set_emoji_status(&status.into(), move |s| {
            success_clone.set(s);
        })
        .unwrap();

        assert!(success.get());
        let stored = Reflect::get(&webapp, &"st".into()).unwrap();
        let id = Reflect::get(&stored, &"custom_emoji_id".into())
            .unwrap()
            .as_string();
        assert_eq!(id.as_deref(), Some("321"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_popup_invokes_callback() {
        let webapp = setup_webapp();
        let show_popup = Function::new_with_args("params, cb", "cb('ok');");
        let _ = Reflect::set(&webapp, &"showPopup".into(), &show_popup);

        let app = TelegramWebApp::instance().unwrap();
        let button = Rc::new(RefCell::new(String::new()));
        let button_clone = Rc::clone(&button);

        app.show_popup(&JsValue::NULL, move |id| {
            *button_clone.borrow_mut() = id;
        })
        .unwrap();

        assert_eq!(button.borrow().as_str(), "ok");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn read_text_from_clipboard_invokes_callback() {
        let webapp = setup_webapp();
        let read_clip = Function::new_with_args("cb", "cb('clip');");
        let _ = Reflect::set(&webapp, &"readTextFromClipboard".into(), &read_clip);

        let app = TelegramWebApp::instance().unwrap();
        let text = Rc::new(RefCell::new(String::new()));
        let text_clone = Rc::clone(&text);

        app.read_text_from_clipboard(move |t| {
            *text_clone.borrow_mut() = t;
        })
        .unwrap();

        assert_eq!(text.borrow().as_str(), "clip");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn scan_qr_popup_invokes_callback_and_close() {
        let webapp = setup_webapp();
        let show_scan = Function::new_with_args("text, cb", "cb('code');");
        let close_scan = Function::new_with_args("", "this.closed = true;");
        let _ = Reflect::set(&webapp, &"showScanQrPopup".into(), &show_scan);
        let _ = Reflect::set(&webapp, &"closeScanQrPopup".into(), &close_scan);

        let app = TelegramWebApp::instance().unwrap();
        let text = Rc::new(RefCell::new(String::new()));
        let text_clone = Rc::clone(&text);

        app.show_scan_qr_popup("scan", move |value| {
            *text_clone.borrow_mut() = value;
        })
        .unwrap();
        assert_eq!(text.borrow().as_str(), "code");

        app.close_scan_qr_popup().unwrap();
        let closed = Reflect::get(&webapp, &"closed".into())
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(closed);
    }
}
