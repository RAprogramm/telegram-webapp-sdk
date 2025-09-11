use telegram_webapp_sdk::core::safe_context::get_context;

use crate::components::{
    display_data::{DisplayDataRow, render_display_data},
    page_layout::PageLayout
};

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

    let section = render_display_data("Theme Params", &rows);
    page.append(&section);
}
