// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use web_sys::{Document, Element, window};

/// Struct representing a basic page layout
pub struct PageLayout {
    pub root: Element
}

impl PageLayout {
    /// Creates a new page layout inside the #app-root element, clears it and
    /// sets page title.
    pub fn new(title: &str) -> Self {
        let document = document();

        document.set_title(title);

        let root = document
            .get_element_by_id("app-root")
            .expect("Expected <div id='app-root'> to exist");

        root.set_inner_html("");

        Self {
            root
        }
    }

    pub fn with_header(title: &str, header: &str) -> Self {
        let layout = Self::new(title);

        let h1 = document()
            .create_element("h1")
            .expect("failed to create <h1>");
        h1.set_inner_html(header);

        layout.root.append_child(&h1).unwrap();
        layout
    }

    pub fn append(&self, element: &Element) {
        let _ = self.root.append_child(element);
    }
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
