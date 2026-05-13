// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::{closure::Closure, JsCast, JsValue};
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
        F: FnMut(web_sys::Event) + 'static,
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
