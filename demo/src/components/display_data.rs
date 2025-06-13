use web_sys::{window, Element};

use crate::components::rgb::RGB; // ← your swatch component

/// A single row in a key/value display list.
pub struct DisplayDataRow {
    pub title: String,
    pub value: String
}

/// Renders a `<section>` with a header and rows of title + value.
/// If a value starts with `#`, we render a little color‐circle before it.
pub fn render_display_data(header: &str, rows: &[DisplayDataRow]) -> Element {
    let document = window().unwrap().document().unwrap();

    // <section class="display-data">
    let section = document.create_element("section").unwrap();
    section.set_class_name("display-data");

    // <h3 class="display-data-header">…</h3>
    let h3 = document.create_element("h3").unwrap();
    h3.set_class_name("display-data-header");
    h3.set_inner_html(header);
    section.append_child(&h3).unwrap();

    for row in rows {
        // <div class="display-data-row">
        let row_el = document.create_element("div").unwrap();
        row_el.set_class_name("display-data-row");

        // title <span class="display-data-title">…</span>
        let title_el = document.create_element("span").unwrap();
        title_el.set_class_name("display-data-title");
        title_el.set_inner_html(&row.title);

        // value <span class="display-data-value">…</span>
        let value_el = document.create_element("span").unwrap();
        value_el.set_class_name("display-data-value");

        if row.value.starts_with('#') {
            let swatch = RGB::render(&row.value);
            value_el.append_child(&swatch).unwrap();
        } else {
            value_el.set_text_content(Some(&row.value));
        }

        row_el.append_child(&title_el).unwrap();
        row_el.append_child(&value_el).unwrap();
        section.append_child(&row_el).unwrap();
    }

    section
}
