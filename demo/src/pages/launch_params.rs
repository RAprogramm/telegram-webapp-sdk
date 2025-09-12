use telegram_webapp_sdk::core::context::get_launch_params;
use wasm_bindgen::JsValue;

use crate::components::{
    display_data::{DisplayDataRow, render_display_data},
    page_layout::PageLayout
};

telegram_page!(
    "/launch-params",
    /// Renders the Launch Parameters page.
    pub fn render_launch_params_page() {
        super::index::clear_app_root();

        let page = PageLayout::new("Launch Parameters");

        let lp = match get_launch_params() {
            Ok(params) => params,
            Err(err) => {
                web_sys::console::error_1(&err);
                return;
            }
        };

        let rows = vec![
            DisplayDataRow {
                title: "tgWebAppPlatform".into(),
                value: lp.tg_web_app_platform.unwrap_or_else(|| "unknown".into())
            },
            DisplayDataRow {
                title: "tgWebAppVersion".into(),
                value: lp.tg_web_app_version.unwrap_or_else(|| "unknown".into())
            },
            DisplayDataRow {
                title: "tgWebAppStartParam".into(),
                value: lp.tg_web_app_start_param.unwrap_or_else(|| "–".into())
            },
            DisplayDataRow {
                title: "tgWebAppShowSettings".into(),
                value: lp.tg_web_app_show_settings.unwrap_or(false).to_string()
            },
            DisplayDataRow {
                title: "tgWebAppBotInline".into(),
                value: lp.tg_web_app_bot_inline.unwrap_or(false).to_string()
            },
        ];

        match render_display_data("Launch Parameters", &rows) {
            Ok(section) => page.append(&section),
            Err(err) => {
                web_sys::console::error_1(&JsValue::from_str(&err.to_string()));
            }
        }
    }
);
