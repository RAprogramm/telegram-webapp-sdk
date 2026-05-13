// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Element, EventTarget, Node};

pub trait ElementExt {
    fn set_class(&self, class: &str);
    fn set_id(&self, id: &str);
    fn set_text(&self, text: &str);
    fn set_html(&self, html: &str) -> Result<(), JsValue>;
    fn set_attr(&self, attr: &str, value: &str) -> Result<(), JsValue>;
    fn get_attr(&self, attr: &str) -> Option<String>;
    fn remove_attr(&self, attr: &str) -> Result<(), JsValue>;
    fn add_class(&self, class: &str) -> Result<(), JsValue>;
    fn remove_class(&self, class: &str) -> Result<(), JsValue>;
    fn toggle_class(&self, class: &str) -> Result<(), JsValue>;
    fn has_class(&self, class: &str) -> bool;
    fn on<F>(&self, event: &str, handler: F) -> Result<(), JsValue>
    where
        F: FnMut(web_sys::Event) + 'static;
    fn append(&self, child: &Element) -> Result<(), JsValue>;
    fn prepend(&self, child: &Element) -> Result<(), JsValue>;
    fn remove(&self) -> Result<(), JsValue>;
    fn clear(&self);
}

impl ElementExt for Element {
    fn set_class(&self, class: &str) {
        self.set_attribute("class", class).ok();
    }

    fn set_id(&self, id: &str) {
        self.set_attribute("id", id).ok();
    }

    fn set_text(&self, text: &str) {
        self.set_text_content(Some(text));
    }

    fn set_html(&self, html: &str) -> Result<(), JsValue> {
        self.set_inner_html(html);
        Ok(())
    }

    fn set_attr(&self, attr: &str, value: &str) -> Result<(), JsValue> {
        self.set_attribute(attr, value)
    }

    fn get_attr(&self, attr: &str) -> Option<String> {
        self.get_attribute(attr)
    }

    fn remove_attr(&self, attr: &str) -> Result<(), JsValue> {
        self.remove_attribute(attr)
    }

    fn add_class(&self, class: &str) -> Result<(), JsValue> {
        let current = self.get_attribute("class").unwrap_or_default();
        if !current.split_whitespace().any(|c| c == class) {
            let new_class = if current.is_empty() {
                class.to_string()
            } else {
                format!("{} {}", current, class)
            };
            self.set_attribute("class", &new_class)?;
        }
        Ok(())
    }

    fn remove_class(&self, class: &str) -> Result<(), JsValue> {
        let current = self.get_attribute("class").unwrap_or_default();
        let new_class: String = current
            .split_whitespace()
            .filter(|c| *c != class)
            .collect::<Vec<_>>()
            .join(" ");
        if new_class.is_empty() {
            self.remove_attribute("class")?;
        } else {
            self.set_attribute("class", &new_class)?;
        }
        Ok(())
    }

    fn toggle_class(&self, class: &str) -> Result<(), JsValue> {
        if self.has_class(class) {
            self.remove_class(class)?;
        } else {
            self.add_class(class)?;
        }
        Ok(())
    }

    fn has_class(&self, class: &str) -> bool {
        self.get_attribute("class")
            .map(|c| c.split_whitespace().any(|x| x == class))
            .unwrap_or(false)
    }

    fn on<F>(&self, event: &str, mut handler: F) -> Result<(), JsValue>
    where
        F: FnMut(web_sys::Event) + 'static
    {
        let target: EventTarget = self.clone().unchecked_into();
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            handler(e);
        }) as Box<dyn FnMut(_)>);

        target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    fn append(&self, child: &Element) -> Result<(), JsValue> {
        Node::append_child(self, child)
            .map(|_| ())
            .map_err(|e| e.unchecked_into())
    }

    fn prepend(&self, child: &Element) -> Result<(), JsValue> {
        if let Some(first) = self.first_child() {
            Node::insert_before(self, child, Some(&first))
                .map(|_| ())
                .map_err(|e| e.unchecked_into())
        } else {
            self.append(child)
        }
    }

    fn remove(&self) -> Result<(), JsValue> {
        if let Some(parent) = self.parent_element() {
            Node::remove_child(&parent, self)
                .map(|_| ())
                .map_err(|e| e.unchecked_into())
        } else {
            Ok(())
        }
    }

    fn clear(&self) {
        while let Some(child) = self.first_child() {
            let _ = Node::remove_child(self, &child);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::Element;

    use super::*;
    use crate::dom::Document;

    wasm_bindgen_test_configure!(run_in_browser);

    fn make(tag: &str) -> Element {
        Document.create_element(tag).expect("create")
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_class_sets_attribute() {
        let el = make("div");
        el.set_class("a b");
        assert_eq!(el.get_attribute("class").as_deref(), Some("a b"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_id_sets_attribute() {
        let el = make("div");
        el.set_id("x");
        assert_eq!(el.get_attribute("id").as_deref(), Some("x"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_text_sets_text_content() {
        let el = make("p");
        el.set_text("hello");
        assert_eq!(el.text_content().as_deref(), Some("hello"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_html_sets_inner_html() {
        let el = make("div");
        el.set_html("<b>bold</b>").expect("ok");
        assert!(el.inner_html().contains("<b>"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_and_get_attr() {
        let el = make("div");
        el.set_attr("data-x", "42").expect("ok");
        assert_eq!(el.get_attr("data-x").as_deref(), Some("42"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn remove_attr_clears_value() {
        let el = make("div");
        el.set_attr("data-x", "42").expect("ok");
        el.remove_attr("data-x").expect("ok");
        assert!(el.get_attr("data-x").is_none());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn add_class_appends_and_dedupes() {
        let el = make("div");
        el.add_class("a").expect("ok");
        el.add_class("b").expect("ok");
        el.add_class("a").expect("ok");
        assert_eq!(el.get_attribute("class").as_deref(), Some("a b"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn remove_class_filters_and_clears_when_empty() {
        let el = make("div");
        el.set_class("a b");
        el.remove_class("a").expect("ok");
        assert_eq!(el.get_attribute("class").as_deref(), Some("b"));
        el.remove_class("b").expect("ok");
        assert!(el.get_attribute("class").is_none());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn toggle_class_round_trip() {
        let el = make("div");
        el.toggle_class("on").expect("ok");
        assert!(el.has_class("on"));
        el.toggle_class("on").expect("ok");
        assert!(!el.has_class("on"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn has_class_finds_only_whole_tokens() {
        let el = make("div");
        el.set_class("alpha beta");
        assert!(el.has_class("alpha"));
        assert!(el.has_class("beta"));
        assert!(!el.has_class("alph"));
        assert!(!el.has_class("gamma"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn on_event_fires_handler() {
        let body = Document.body().expect("body");
        let el = make("button");
        body.append_child(&el).expect("attach");

        let hits = Rc::new(Cell::new(0u32));
        let hits_cb = hits.clone();
        el.on("click", move |_| hits_cb.set(hits_cb.get() + 1))
            .expect("ok");

        let evt = web_sys::Event::new("click").expect("event");
        el.dispatch_event(&evt).expect("dispatch");

        assert_eq!(hits.get(), 1);
        el.remove();
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn append_places_child_at_end() {
        let parent = make("ul");
        let a = make("li");
        a.set_id("a");
        let b = make("li");
        b.set_id("b");
        parent.append(&a).expect("ok");
        parent.append(&b).expect("ok");

        let last = parent.last_element_child().expect("last");
        assert_eq!(last.id(), "b");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn prepend_inserts_at_start_or_appends_when_empty() {
        let parent = make("ul");
        let a = make("li");
        a.set_id("a");
        parent.prepend(&a).expect("ok");
        assert_eq!(parent.first_element_child().expect("first").id(), "a");

        let b = make("li");
        b.set_id("b");
        parent.prepend(&b).expect("ok");
        assert_eq!(parent.first_element_child().expect("first").id(), "b");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn remove_detaches_and_is_noop_without_parent() {
        let parent = make("div");
        let child = make("span");
        parent.append(&child).expect("ok");
        assert!(parent.first_element_child().is_some());
        // UFCS because `web_sys::Element::remove` also exists and shadows the trait
        // method.
        ElementExt::remove(&child).expect("detach");
        assert!(parent.first_element_child().is_none());

        let orphan = make("div");
        ElementExt::remove(&orphan).expect("noop");
    }

    fn element_child_count(parent: &Element) -> usize {
        let mut count = 0usize;
        let mut next = parent.first_element_child();
        while let Some(el) = next {
            count += 1;
            next = el.next_element_sibling();
        }
        count
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn clear_removes_all_children() {
        let parent = make("div");
        for _ in 0..3 {
            parent.append(&make("span")).expect("ok");
        }
        assert_eq!(element_child_count(&parent), 3);
        parent.clear();
        assert_eq!(element_child_count(&parent), 0);
    }
}
