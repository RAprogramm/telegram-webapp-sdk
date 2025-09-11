use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

/// A little UI “component” that renders “🔵 #112233”
/// as `<span class="rgb"><i class="rgb__icon"
/// style="background:#112233"></i>#112233</span>`.
pub struct RGB;

impl RGB {
    /// Build the DOM node for a given `#RRGGBB` string.
    pub fn render(color: &str) -> HtmlElement {
        let doc = window().unwrap().document().unwrap();

        // <span class="rgb">
        let span = doc
            .create_element("span")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        span.set_class_name("rgb");

        //   <i class="rgb__icon" style="background-color: #RRGGBB"></i>
        let icon = doc
            .create_element("i")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        icon.set_class_name("rgb__icon");
        icon.style()
            .set_property("background-color", color)
            .unwrap();
        span.append_child(&icon).unwrap();

        //   text node: “#RRGGBB”
        let text = doc.create_text_node(color);
        span.append_child(&text).unwrap();

        span
    }
}
