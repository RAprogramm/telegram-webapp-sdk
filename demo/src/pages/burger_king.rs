use telegram_webapp_sdk::{logger, webapp::TelegramWebApp};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{Document, Element, HtmlElement, window};

use crate::components::page_layout::PageLayout;

/// Represents a single menu item in the Burger King demo.
#[derive(Clone, Debug, PartialEq)]
struct MenuItem {
    id:          u32,
    name:        &'static str,
    price_cents: u32
}

impl MenuItem {
    /// Build JSON payload describing the item.
    fn payload(&self) -> String {
        format!(
            r#"{{"id":{},"name":"{}","price_cents":{}}}"#,
            self.id, self.name, self.price_cents
        )
    }
}

telegram_page!(
    "/burger-king",
    /// Render Burger King menu page with order buttons.
    pub fn render_burger_king_page() {
        let page = PageLayout::with_header("Burger King Demo", "Burger King Menu");

        let items = [
            MenuItem {
                id:          1,
                name:        "Whopper",
                price_cents: 599
            },
            MenuItem {
                id:          2,
                name:        "Cheeseburger",
                price_cents: 299
            },
            MenuItem {
                id:          3,
                name:        "Chicken Nuggets",
                price_cents: 399
            }
        ];

        for item in &items {
            match render_item(item) {
                Ok(el) => page.append(&el),
                Err(err) => logger::error(&format!("render_item failed: {:?}", err))
            }
        }
    }
);

fn render_item(item: &MenuItem) -> Result<Element, JsValue> {
    let document = document()?;
    let container = document.create_element("div")?;
    container.set_class_name("menu-item");

    let label = document.create_element("span")?;
    label.set_inner_html(&format!(
        "{} - ${:.2}",
        item.name,
        item.price_cents as f64 / 100.0
    ));
    container.append_child(&label)?;

    let button = document.create_element("button")?;
    button.set_inner_html("Order");
    let button_el: HtmlElement = button.clone().dyn_into()?;

    let item_clone = item.clone();
    let click = Closure::<dyn FnMut()>::new(move || {
        if let Some(app) = TelegramWebApp::instance() {
            if let Err(err) = app.send_data(&item_clone.payload()) {
                logger::error(&format!("send_data failed: {:?}", err));
            }
        } else {
            logger::error("Telegram WebApp instance not found");
        }
    });
    button_el.set_onclick(Some(click.as_ref().unchecked_ref()));
    click.forget();

    container.append_child(&button_el)?;
    Ok(container)
}

fn document() -> Result<Document, JsValue> {
    window()
        .ok_or_else(|| JsValue::from_str("window not available"))?
        .document()
        .ok_or_else(|| JsValue::from_str("document not available"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_is_valid() {
        let item = MenuItem {
            id:          7,
            name:        "Test",
            price_cents: 1234
        };
        assert_eq!(
            item.payload(),
            "{\"id\":7,\"name\":\"Test\",\"price_cents\":1234}".to_string()
        );
    }
}
