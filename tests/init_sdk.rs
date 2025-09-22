#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect};
use telegram_webapp_sdk::core::{context::TelegramContext, init::init_sdk};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::window;

wasm_bindgen_test_configure!(run_in_browser);

fn install_webapp(init_data: &str) -> Result<(), JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let telegram = Object::new();
    let webapp = Object::new();

    Reflect::set(&webapp, &"initData".into(), &JsValue::from_str(init_data))?;
    Reflect::set(
        &webapp,
        &"themeParams".into(),
        &JsValue::from(Object::new())
    )?;

    Reflect::set(&telegram, &"WebApp".into(), &webapp)?;
    Reflect::set(&win, &"Telegram".into(), &telegram)?;

    Ok(())
}

#[wasm_bindgen_test]
fn init_sdk_propagates_query_id() -> Result<(), JsValue> {
    install_webapp("query_id=inline-123&auth_date=1&hash=abc")?;

    init_sdk()?;

    let query_id = TelegramContext::get(|ctx| ctx.init_data.query_id.as_deref())
        .ok_or_else(|| JsValue::from_str("context not initialized"))?;

    assert_eq!(query_id, Some("inline-123"));

    Ok(())
}
