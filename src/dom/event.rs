// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::Element;

pub struct EventHandler;

impl EventHandler {
    pub fn on<F>(element: &Element, event: &str, handler: F) -> Result<(), JsValue>
    where
        F: FnMut(web_sys::Event) + 'static
    {
        let target: web_sys::EventTarget = element.clone().unchecked_into();
        let closure = Closure::wrap(handler.into());

        target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    pub fn on_click<F>(element: &Element, mut handler: F) -> Result<(), JsValue>
    where
        F: FnMut(web_sys::MouseEvent) + 'static
    {
        let target: web_sys::EventTarget = element.clone().unchecked_into();
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Ok(mouse) = e.dyn_into::<web_sys::MouseEvent>() {
                handler(mouse);
            }
        }) as Box<dyn FnMut(_)>);

        target.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}
