use telegram_webapp_sdk::{
    logger::info,
    webapp::{BottomButton, TelegramWebApp}
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, window};

const BUTTON_IDS: &[(&str, fn(&TelegramWebApp))] = &[
    ("send-data", |tg| tg.send_data("Hello from Dev Menu!")),
    ("expand", |tg| tg.expand()),
    ("close", |tg| tg.close()),
    ("alert", |tg| {
        tg.show_alert("This is a test alert from DevMenu")
    }),
    ("main-button", |tg| {
        tg.set_bottom_button_text(BottomButton::Main, "Clicked!");
        tg.show_bottom_button(BottomButton::Main);
    }),
    ("is-expanded", |tg| {
        let expanded = tg.is_expanded();
        info(&format!("isExpanded = {}", expanded));
        if expanded {
            tg.show_alert("Viewport is already expanded.");
        } else {
            tg.expand();
        }
    })
];

pub fn setup_dev_menu() {
    let doc = match window().and_then(|w| w.document()) {
        Some(doc) => doc,
        None => return
    };

    for (id, handler) in BUTTON_IDS {
        if let Some(elem) = doc
            .get_element_by_id(id)
            .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        {
            let handler = handler.clone();
            let cb = Closure::<dyn FnMut()>::new(move || {
                if let Some(tg) = TelegramWebApp::instance() {
                    handler(&tg);
                }
            });
            elem.set_onclick(Some(cb.as_ref().unchecked_ref()));
            cb.forget(); // JS leak is OK for dev tools
        }
    }
}
