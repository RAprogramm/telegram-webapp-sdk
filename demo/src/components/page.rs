use web_sys::{Document, Element, window};

use crate::components::{nav_link::nav_link, page_layout::PageLayout};

/// Creates a page with a header and list of children
pub fn render_page(title: &str, children: Vec<Element>, back: Option<&str>) -> PageLayout {
    let layout = PageLayout::new(title);
    let _document = document();

    // Optional back button
    if let Some(href) = back {
        let back_link = nav_link("← Back", None, href);
        layout.append(&back_link);
    }

    for child in children {
        layout.append(&child);
    }

    layout
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
