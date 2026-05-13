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

#[cfg(test)]
mod tests {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::Element;

    use super::*;
    use crate::dom::ElementExt;

    wasm_bindgen_test_configure!(run_in_browser);

    fn fresh_root(id: &str) -> Element {
        let doc = Doc;
        let body = doc.body().expect("body");
        let container = doc.create_element("div").expect("create container");
        container.set_id(id);
        body.append_child(&container).expect("append container");
        container
    }

    fn cleanup(root: &Element) {
        root.remove();
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn create_element_returns_tagged() {
        let el = Doc.create_element("section").expect("create");
        assert_eq!(el.tag_name().to_ascii_lowercase(), "section");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn get_element_by_id_finds_existing_and_misses() {
        let root = fresh_root("doc-test-get-by-id");
        assert!(Doc.get_element_by_id("doc-test-get-by-id").is_some());
        assert!(Doc.get_element_by_id("doc-test-absent").is_none());
        cleanup(&root);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn query_selector_finds_and_misses() {
        let root = fresh_root("doc-test-qs");
        let child = Doc.create_element("span").expect("span");
        child.set_class("hit");
        root.append_child(&child).expect("append");

        let found = Doc.query_selector("#doc-test-qs .hit").expect("ok");
        assert!(found.is_some());
        let absent = Doc.query_selector("#doc-test-qs .nope").expect("ok");
        assert!(absent.is_none());

        cleanup(&root);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn query_selector_invalid_returns_err() {
        assert!(Doc.query_selector(">>>broken<<<").is_err());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn body_returns_html_element() {
        let body = Doc.body().expect("body");
        let as_el: &Element = body.unchecked_ref();
        assert_eq!(as_el.tag_name().to_ascii_lowercase(), "body");
    }
}
