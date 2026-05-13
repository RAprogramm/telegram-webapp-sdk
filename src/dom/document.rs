// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlElement};

pub struct Doc;

impl Doc {
    pub fn get_element_by_id(&self, id: &str) -> Option<Element> {
        web_sys::window()?.document()?.get_element_by_id(id)
    }

    pub fn query_selector(&self, selector: &str) -> Result<Option<Element>, JsValue> {
        web_sys::window()
            .and_then(|w| w.document())
            .ok_or_else(|| JsValue::from_str("document not available"))?
            .query_selector(selector)
    }

    pub fn create_element(&self, tag: &str) -> Result<Element, JsValue> {
        web_sys::window()
            .ok_or_else(|| JsValue::from_str("window not available"))?
            .document()
            .ok_or_else(|| JsValue::from_str("document not available"))?
            .create_element(tag)
    }

    pub fn body(&self) -> Result<HtmlElement, JsValue> {
        web_sys::window()
            .ok_or_else(|| JsValue::from_str("window not available"))?
            .document()
            .ok_or_else(|| JsValue::from_str("document not available"))?
            .body()
            .ok_or_else(|| JsValue::from_str("body not available"))
    }
}

impl Default for Doc {
    fn default() -> Self {
        Self
    }
}
