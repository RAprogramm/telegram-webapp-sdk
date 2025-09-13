#![cfg(all(target_arch = "wasm32", feature = "macros"))]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn telegram_button_creates_button() {
    let document = web_sys::window().unwrap().document().unwrap();
    let button =
        telegram_webapp_sdk::telegram_button!(document, "OK", class = "btn", "type" = "button")
            .unwrap();
    assert_eq!(button.tag_name(), "BUTTON");
    assert_eq!(button.class_name(), "btn");
    assert_eq!(button.get_attribute("type").unwrap(), "button");
}

#[wasm_bindgen_test]
fn telegram_image_creates_image() {
    let document = web_sys::window().unwrap().document().unwrap();
    let img = telegram_webapp_sdk::telegram_image!(
        document,
        "https://example.com/logo.png",
        class = "pic",
        alt = "Logo"
    )
    .unwrap();
    assert_eq!(img.tag_name(), "IMG");
    assert_eq!(img.class_name(), "pic");
    assert_eq!(
        img.get_attribute("src").unwrap(),
        "https://example.com/logo.png"
    );
    assert_eq!(img.get_attribute("alt").unwrap(), "Logo");
}
