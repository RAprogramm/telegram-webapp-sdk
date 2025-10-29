// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::webapp::TelegramWebApp;

impl TelegramWebApp {
    /// Call `WebApp.showAlert(message)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_alert(&self, msg: &str) -> Result<(), JsValue> {
        self.call1("showAlert", &msg.into())
    }

    /// Call `WebApp.showConfirm(message, callback)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_confirm<F>(&self, msg: &str, on_confirm: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(bool)>::new(on_confirm);
        let f = Reflect::get(&self.inner, &"showConfirm".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("showConfirm is not a function"))?;
        func.call2(&self.inner, &msg.into(), cb.as_ref().unchecked_ref())?;
        cb.forget(); // safe leak for JS lifetime
        Ok(())
    }

    /// Call `WebApp.showPopup(params, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.show_popup(&params.into(), |id| {
    ///     let _ = id;
    /// })
    /// .unwrap();
    /// ```
    pub fn show_popup<F>(&self, params: &JsValue, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |id: JsValue| {
            callback(id.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"showPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, params, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.showScanQrPopup(text, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.show_scan_qr_popup("Scan", |text| {
    ///     let _ = text;
    /// })
    /// .unwrap();
    /// ```
    pub fn show_scan_qr_popup<F>(&self, text: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| {
            callback(value.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"showScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &text.into(), cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.closeScanQrPopup()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.close_scan_qr_popup().unwrap();
    /// ```
    pub fn close_scan_qr_popup(&self) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"closeScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call0(&self.inner)?;
        Ok(())
    }
}
