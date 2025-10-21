// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::{AppError, AppResult};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

/// A little UI “component” that renders “🔵 #112233”
/// as `<span class="rgb"><i class="rgb__icon"
/// style="background:#112233"></i>#112233</span>`.
pub struct RGB;

impl RGB {
    /// Build the DOM node for a given `#RRGGBB` string.
    pub fn render(color: &str) -> AppResult<HtmlElement> {
        let doc = window()
            .ok_or_else(|| AppError::internal("no window"))?
            .document()
            .ok_or_else(|| AppError::internal("no document"))?;

        // <span class="rgb">
        let span = doc
            .create_element("span")
            .map_err(|_| AppError::internal("create span"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| AppError::internal("span into HtmlElement"))?;
        span.set_class_name("rgb");

        //   <i class="rgb__icon" style="background-color: #RRGGBB"></i>
        let icon = doc
            .create_element("i")
            .map_err(|_| AppError::internal("create icon"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| AppError::internal("icon into HtmlElement"))?;
        icon.set_class_name("rgb__icon");
        icon.style()
            .set_property("background-color", color)
            .map_err(|_| AppError::internal("set color"))?;
        span.append_child(&icon)
            .map_err(|_| AppError::internal("append icon"))?;

        //   text node: “#RRGGBB”
        let text = doc.create_text_node(color);
        span.append_child(&text)
            .map_err(|_| AppError::internal("append text"))?;

        Ok(span)
    }
}
