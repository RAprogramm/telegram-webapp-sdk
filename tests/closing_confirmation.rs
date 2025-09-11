use std::{cell::Cell, rc::Rc};

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::webapp::TelegramWebApp;
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::window;

fn setup_webapp() -> Object {
    let win = window().unwrap();
    let telegram = Object::new();
    let webapp = Object::new();
    let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
    let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
    webapp
}

#[wasm_bindgen_test]
fn enable_closing_confirmation_calls_js() {
    let webapp = setup_webapp();
    let called = Rc::new(Cell::new(false));
    let called_clone = Rc::clone(&called);

    let cb = Closure::<dyn FnMut()>::new(move || {
        called_clone.set(true);
    });
    let _ = Reflect::set(
        &webapp,
        &"enableClosingConfirmation".into(),
        cb.as_ref().unchecked_ref()
    );
    cb.forget();

    let app = TelegramWebApp::instance().unwrap();
    app.enable_closing_confirmation().unwrap();
    assert!(called.get());
}

#[wasm_bindgen_test]
fn disable_closing_confirmation_calls_js() {
    let webapp = setup_webapp();
    let called = Rc::new(Cell::new(false));
    let called_clone = Rc::clone(&called);

    let cb = Closure::<dyn FnMut()>::new(move || {
        called_clone.set(true);
    });
    let _ = Reflect::set(
        &webapp,
        &"disableClosingConfirmation".into(),
        cb.as_ref().unchecked_ref()
    );
    cb.forget();

    let app = TelegramWebApp::instance().unwrap();
    app.disable_closing_confirmation().unwrap();
    assert!(called.get());
}
