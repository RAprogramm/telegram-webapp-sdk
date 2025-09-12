use telegram_webapp_sdk::core::safe_context::get_context;
use wasm_bindgen::JsValue;

use crate::components::{
    display_data::{DisplayDataRow, render_display_data},
    page_layout::PageLayout
};

/// Renders the Init Data page
pub fn render_init_data_page() {
    let layout = PageLayout::new("Init Data");

    let result = get_context(|ctx| {
        let mut rows = vec![];

        if let Some(user) = &ctx.init_data.user {
            rows.push(DisplayDataRow {
                title: "id".into(),
                value: user.id.to_string()
            });
            rows.push(DisplayDataRow {
                title: "username".into(),
                value: user.username.clone().unwrap_or_default()
            });
            rows.push(DisplayDataRow {
                title: "language".into(),
                value: user.language_code.clone().unwrap_or_default()
            });
        }

        Some(rows)
    });

    match result {
        Ok(Some(rows)) => match render_display_data("User", &rows) {
            Ok(section) => layout.append(&section),
            Err(err) => {
                web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
            }
        },
        _ => {
            match render_display_data(
                "User",
                &[DisplayDataRow {
                    title: "error".into(),
                    value: "init_data not available".into()
                }]
            ) {
                Ok(fallback) => layout.append(&fallback),
                Err(err) => {
                    web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
                }
            }
        }
    }
}
