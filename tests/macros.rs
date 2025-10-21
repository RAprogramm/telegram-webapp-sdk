// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![cfg(all(target_arch = "wasm32", feature = "macros"))]

use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn telegram_button_creates_button() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("no document"))?;
    let button =
        telegram_webapp_sdk::telegram_button!(document, "OK", class = "btn", "type" = "button")?;
    assert_eq!(button.tag_name(), "BUTTON");
    assert_eq!(button.class_name(), "btn");
    let btn_type = button
        .get_attribute("type")
        .ok_or_else(|| JsValue::from_str("missing type attribute"))?;
    assert_eq!(btn_type, "button");
    Ok(())
}

#[wasm_bindgen_test]
fn telegram_image_creates_image() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("no document"))?;
    let img = telegram_webapp_sdk::telegram_image!(
        document,
        "https://example.com/logo.png",
        class = "pic",
        alt = "Logo"
    )?;
    assert_eq!(img.tag_name(), "IMG");
    assert_eq!(img.class_name(), "pic");
    let src = img
        .get_attribute("src")
        .ok_or_else(|| JsValue::from_str("missing src"))?;
    assert_eq!(src, "https://example.com/logo.png");
    let alt = img
        .get_attribute("alt")
        .ok_or_else(|| JsValue::from_str("missing alt"))?;
    assert_eq!(alt, "Logo");
    Ok(())
}
