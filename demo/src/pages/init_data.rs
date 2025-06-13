use telegram_webapp_sdk::core::safe_context::get_context;

use crate::components::{
    display_data::{render_display_data, DisplayDataRow},
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
        Ok(Some(rows)) => {
            let section = render_display_data("User", &rows);
            layout.append(&section);
        }
        _ => {
            let fallback = render_display_data(
                "User",
                &[DisplayDataRow {
                    title: "error".into(),
                    value: "init_data not available".into()
                }]
            );
            layout.append(&fallback);
        }
    }
}
