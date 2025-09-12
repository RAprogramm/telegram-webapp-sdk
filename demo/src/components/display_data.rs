use masterror::{AppError, AppResult};
use web_sys::{Element, window};

use crate::components::rgb::RGB; // ← your swatch component

/// A single row in a key/value display list.
pub struct DisplayDataRow {
    pub title: String,
    pub value: String
}

/// Renders a `<section>` with a header and rows of title + value.
/// If a value starts with `#`, we render a little color‐circle before it.
pub fn render_display_data(header: &str, rows: &[DisplayDataRow]) -> AppResult<Element> {
    let document = window()
        .ok_or_else(|| AppError::internal("no window"))?
        .document()
        .ok_or_else(|| AppError::internal("no document"))?;

    // <section class="display-data">
    let section = document
        .create_element("section")
        .map_err(|_| AppError::internal("create section"))?;
    section.set_class_name("display-data");

    // <h3 class="display-data-header">…</h3>
    let h3 = document
        .create_element("h3")
        .map_err(|_| AppError::internal("create h3"))?;
    h3.set_class_name("display-data-header");
    h3.set_inner_html(header);
    section
        .append_child(&h3)
        .map_err(|_| AppError::internal("append h3"))?;

    for row in rows {
        // <div class="display-data-row">
        let row_el = document
            .create_element("div")
            .map_err(|_| AppError::internal("create row"))?;
        row_el.set_class_name("display-data-row");

        // title <span class="display-data-title">…</span>
        let title_el = document
            .create_element("span")
            .map_err(|_| AppError::internal("create title"))?;
        title_el.set_class_name("display-data-title");
        title_el.set_inner_html(&row.title);

        // value <span class="display-data-value">…</span>
        let value_el = document
            .create_element("span")
            .map_err(|_| AppError::internal("create value"))?;
        value_el.set_class_name("display-data-value");

        if row.value.starts_with('#') {
            let swatch = RGB::render(&row.value)?;
            value_el
                .append_child(&swatch)
                .map_err(|_| AppError::internal("append swatch"))?;
        } else {
            value_el.set_text_content(Some(&row.value));
        }

        row_el
            .append_child(&title_el)
            .map_err(|_| AppError::internal("append title"))?;
        row_el
            .append_child(&value_el)
            .map_err(|_| AppError::internal("append value"))?;
        section
            .append_child(&row_el)
            .map_err(|_| AppError::internal("append row"))?;
    }

    Ok(section)
}
