// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use telegram_webapp_sdk::{
    core::context::TelegramContext,
    dom::{Document, ElementExt},
};

pub fn mount() {
    let doc = Document;

    let root = doc.create_element("div").unwrap();
    root.set_class("container");

    let header = doc.create_element("h1").unwrap();
    header.set_text("Telegram SDK Vanilla Demo");
    header.set_class("title");
    let _ = root.append(&header);

    let user_info = build_user_card(&doc);
    let _ = root.append(&user_info);

    let counter_section = build_counter();
    let _ = root.append(&counter_section);

    let _ = doc.body().unwrap().append(&root);
}

fn build_user_card(doc: &Document) -> web_sys::Element {
    let card = doc.create_element("div").unwrap();
    card.set_class("card user-card");

    let ctx = TelegramContext::get(|ctx| ctx.clone());
    let (name, username, is_premium) =
        if let Some(user) = ctx.as_ref().and_then(|c| c.init_data.user.as_ref()) {
            (
                format!(
                    "{} {}",
                    user.first_name,
                    user.last_name.as_deref().unwrap_or("")
                ),
                user.username.clone().map(|u| format!("@{}", u)),
                user.is_premium.unwrap_or(false),
            )
        } else {
            ("Guest".to_string(), None, false)
        };

    let name_el = doc.create_element("div").unwrap();
    name_el.set_class("user-name");
    name_el.set_text(&name);
    let _ = card.append(&name_el);

    if let Some(un) = username {
        let un_el = doc.create_element("div").unwrap();
        un_el.set_class("user-username");
        un_el.set_text(&un);
        let _ = card.append(&un_el);
    }

    if is_premium {
        let badge = doc.create_element("span").unwrap();
        badge.set_class("badge premium");
        badge.set_text("Premium");
        let _ = card.append(&badge);
    }

    card
}

fn build_counter() -> web_sys::Element {
    let doc = Document;

    let section = doc.create_element("div").unwrap();
    section.set_class("counter-section");

    let label = doc.create_element("div").unwrap();
    label.set_class("counter-label");
    label.set_text("Counter");
    let _ = section.append(&label);

    let value = doc.create_element("div").unwrap();
    value.set_class("counter-value");
    value.set_id("counter-value");
    value.set_text("0");
    let _ = section.append(&value);

    let btn_container = doc.create_element("div").unwrap();
    btn_container.set_class("button-row");

    let inc_btn = doc.create_element("button").unwrap();
    inc_btn.set_class("btn btn-primary");
    inc_btn.set_text("+");
    let _ = inc_btn.on("click", |_| {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("counter-value"))
        {
            let text = el.text_content().unwrap_or_default();
            let current: i32 = text.parse().unwrap_or(0);
            el.set_text(&format!("{}", current + 1));
        }
    });
    let _ = btn_container.append(&inc_btn);

    let dec_btn = doc.create_element("button").unwrap();
    dec_btn.set_class("btn btn-secondary");
    dec_btn.set_text("-");
    let _ = dec_btn.on("click", |_| {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("counter-value"))
        {
            let text = el.text_content().unwrap_or_default();
            let current: i32 = text.parse().unwrap_or(0);
            el.set_text(&format!("{}", current - 1));
        }
    });
    let _ = btn_container.append(&dec_btn);

    let _ = section.append(&btn_container);

    section
}
