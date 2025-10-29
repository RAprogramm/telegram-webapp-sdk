// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::{core::types::download_file_params::DownloadFileParams, webapp::TelegramWebApp};

impl TelegramWebApp {
    /// Call `WebApp.requestWriteAccess(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.request_write_access(|granted| {
    ///     let _ = granted;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_write_access<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        self.call1("requestWriteAccess", cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.requestEmojiStatusAccess(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.request_emoji_status_access(|granted| {
    ///     let _ = granted;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_emoji_status_access<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"requestEmojiStatusAccess".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("requestEmojiStatusAccess is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.setEmojiStatus(status, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use js_sys::Reflect;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let status = Object::new();
    /// let _ = Reflect::set(&status, &"custom_emoji_id".into(), &"123".into());
    /// app.set_emoji_status(&status.into(), |success| {
    ///     let _ = success;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_emoji_status<F>(&self, status: &JsValue, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"setEmojiStatus".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("setEmojiStatus is not a function"))?;
        func.call2(&self.inner, status, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.openInvoice(url, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_invoice("https://invoice", |status| {
    ///     let _ = status;
    /// })
    /// .unwrap();
    /// ```
    pub fn open_invoice<F>(&self, url: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |status: JsValue| {
            callback(status.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"openInvoice".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &url.into(), cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.downloadFile(params, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::core::types::download_file_params::DownloadFileParams;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = DownloadFileParams {
    ///     url:       "https://example.com/file",
    ///     file_name: None,
    ///     mime_type: None
    /// };
    /// app.download_file(params, |file_id| {
    ///     let _ = file_id;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails or the parameters
    /// fail to serialize.
    pub fn download_file<F>(
        &self,
        params: DownloadFileParams<'_>,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let js_params =
            to_value(&params).map_err(|e| JsValue::from_str(&format!("serialize params: {e}")))?;
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"downloadFile".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &js_params, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.readTextFromClipboard(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.read_text_from_clipboard(|text| {
    ///     let _ = text;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn read_text_from_clipboard<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |text: JsValue| {
            callback(text.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"readTextFromClipboard".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("readTextFromClipboard is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }
}
