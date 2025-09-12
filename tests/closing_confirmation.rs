#![cfg(target_arch = "wasm32")]

use std::{cell::Cell, rc::Rc};

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::webapp::TelegramWebApp;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::window;

fn setup_webapp() -> Result<Object, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let telegram = Object::new();
    let webapp = Object::new();
    Reflect::set(&win, &"Telegram".into(), &telegram)?;
    Reflect::set(&telegram, &"WebApp".into(), &webapp)?;
    Ok(webapp)
}

#[wasm_bindgen_test]
fn enable_closing_confirmation_calls_js() -> Result<(), JsValue> {
    let webapp = setup_webapp()?;
    let called = Rc::new(Cell::new(false));
    let called_clone = Rc::clone(&called);

    let cb = Closure::<dyn FnMut()>::new(move || {
        called_clone.set(true);
    });
    Reflect::set(
        &webapp,
        &"enableClosingConfirmation".into(),
        cb.as_ref().unchecked_ref()
    )?;
    cb.forget();

    let app = TelegramWebApp::try_instance()?;
    app.enable_closing_confirmation()?;
    assert!(called.get());
    Ok(())
}

#[wasm_bindgen_test]
fn disable_closing_confirmation_calls_js() -> Result<(), JsValue> {
    let webapp = setup_webapp()?;
    let called = Rc::new(Cell::new(false));
    let called_clone = Rc::clone(&called);

    let cb = Closure::<dyn FnMut()>::new(move || {
        called_clone.set(true);
    });
    Reflect::set(
        &webapp,
        &"disableClosingConfirmation".into(),
        cb.as_ref().unchecked_ref()
    )?;
    cb.forget();

    let app = TelegramWebApp::try_instance()?;
    app.disable_closing_confirmation()?;
    assert!(called.get());
    Ok(())
}

#[wasm_bindgen_test]
fn is_closing_confirmation_enabled_reflects_js() -> Result<(), JsValue> {
    let webapp = setup_webapp()?;
    Reflect::set(
        &webapp,
        &"isClosingConfirmationEnabled".into(),
        &JsValue::TRUE
    )?;

    let app = TelegramWebApp::try_instance()?;
    assert!(app.is_closing_confirmation_enabled());

    Reflect::set(
        &webapp,
        &"isClosingConfirmationEnabled".into(),
        &JsValue::FALSE
    )?;
    assert!(!app.is_closing_confirmation_enabled());
    Ok(())
}
