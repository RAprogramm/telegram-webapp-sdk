use telegram_webapp_sdk::core::safe_context::get_context;
use wasm_bindgen::JsValue;

use crate::components::{
    display_data::{DisplayDataRow, render_display_data},
    page_layout::PageLayout
};

telegram_page!(
    "/theme-params",
    /// Renders the Theme Parameters page.
    pub fn render_theme_params_page() {
        super::index::clear_app_root();

        let page = PageLayout::new("Theme Parameters");

        let rows: Vec<DisplayDataRow> = get_context(|ctx| {
            ctx.theme_params
                .to_map()
                .into_iter()
                .map(|(key, value)| DisplayDataRow {
                    title: key,
                    value
                })
                .collect()
        })
        .unwrap_or_else(|_| {
            vec![DisplayDataRow {
                title: "Error".into(),
                value: "Failed to load theme params".into()
            }]
        });

        match render_display_data("Theme Params", &rows) {
            Ok(section) => page.append(&section),
            Err(err) => {
                web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
            }
        }
    }
);
