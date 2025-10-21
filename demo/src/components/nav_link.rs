// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Document, Element, HtmlElement, window};

/// Creates a navigation link (styled as a cell with optional subtitle).
///
/// # Params
/// - `label`: основной текст (например, "Init Data")
/// - `subtitle`: второстепенный текст (может быть `None`)
/// - `href`: путь, на который должна перейти страница при клике
pub fn nav_link(label: &str, subtitle: Option<&str>, href: &str) -> Element {
    let document = document();
    let link = document.create_element("div").unwrap();
    link.set_class_name("nav-link");

    let title = document.create_element("div").unwrap();
    title.set_class_name("label");
    title.set_inner_html(label);
    link.append_child(&title).unwrap();

    if let Some(sub) = subtitle {
        let subtitle_el = document.create_element("div").unwrap();
        subtitle_el.set_class_name("subtitle");
        subtitle_el.set_inner_html(sub);
        link.append_child(&subtitle_el).unwrap();
    }

    let link_closure = {
        let href = href.to_string();
        Closure::<dyn FnMut()>::new(move || {
            if let Some(window) = window() {
                let _ = window.location().set_href(&href);
            }
        })
    };

    let html_elem: HtmlElement = link.clone().dyn_into().unwrap();
    html_elem.set_onclick(Some(link_closure.as_ref().unchecked_ref()));
    link_closure.forget(); // Safe leak

    link
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
