use telegram_webapp_sdk::telegram_page;
use web_sys::Element;

use crate::components::{nav_link::nav_link, page_layout::PageLayout};

/// Renders the index (home) page with navigation links.
#[telegram_page("/")]
pub fn render_index_page() {
    clear_app_root();

    let page = PageLayout::new("Telegram WebApp SDK Demo");

    let features_header = section_header("Features");
    page.append(&features_header);
    page.append(&nav_link(
        "TON Connect",
        Some("Connect your TON wallet"),
        "/ton-connect" // пока не реализовано, просто пример
    ));
    page.append(&nav_link(
        "Burger King Demo",
        Some("Order burgers via Telegram"),
        "/burger-king"
    ));

    let app_data_header = section_header("Application Launch Data");
    page.append(&app_data_header);
    page.append(&nav_link(
        "Init Data",
        Some("User data, chat information, technical data"),
        "/init-data"
    ));
    page.append(&nav_link(
        "Launch Parameters",
        Some("Platform identifier, Mini Apps version, etc."),
        "/launch-params"
    ));
    page.append(&nav_link(
        "Theme Parameters",
        Some("Telegram application palette information"),
        "/theme-params"
    ));
}

/// Clears the `#app-root` container before rendering a page.
pub fn clear_app_root() {
    if let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Some(root) = document.get_element_by_id("app-root")
    {
        root.set_inner_html("");
    }
}

/// Creates a section header `<h2>`
fn section_header(text: &str) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let el = document.create_element("h2").unwrap();
    el.set_inner_html(text);
    el.set_class_name("section-header");
    el
}
