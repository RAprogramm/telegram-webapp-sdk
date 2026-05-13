// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::{
    core::types::download_file_params::DownloadFileParams,
    webapp::{
        TelegramWebApp,
        core::{await_one_shot, one_shot_promise}
    }
};

impl TelegramWebApp {
    /// Callback variant of [`Self::request_write_access`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_write_access_with_callback<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        self.call1("requestWriteAccess", &cb)
    }

    /// Async wrapper over `WebApp.requestWriteAccess`.
    ///
    /// Resolves with `true` when the user grants permission to receive
    /// messages from the bot.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
    /// let app = TelegramWebApp::try_instance()?;
    /// let granted: bool = app.request_write_access().await?;
    /// let _ = granted;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn request_write_access(&self) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |granted: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &granted);
            });
            let f = Reflect::get(&webapp, &"requestWriteAccess".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("requestWriteAccess is not a function"))?;
            func.call1(&webapp, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Callback variant of [`Self::request_emoji_status_access`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_emoji_status_access_with_callback<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"requestEmojiStatusAccess".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("requestEmojiStatusAccess is not a function"))?;
        func.call1(&self.inner, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.requestEmojiStatusAccess`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn request_emoji_status_access(&self) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |granted: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &granted);
            });
            let f = Reflect::get(&webapp, &"requestEmojiStatusAccess".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("requestEmojiStatusAccess is not a function"))?;
            func.call1(&webapp, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Callback variant of [`Self::set_emoji_status`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_emoji_status_with_callback<F>(
        &self,
        status: &JsValue,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"setEmojiStatus".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("setEmojiStatus is not a function"))?;
        func.call2(&self.inner, status, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.setEmojiStatus`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn set_emoji_status(&self, status: &JsValue) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let status = status.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |v: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &v);
            });
            let f = Reflect::get(&webapp, &"setEmojiStatus".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("setEmojiStatus is not a function"))?;
            func.call2(&webapp, &status, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Callback variant of [`Self::open_invoice`].
    pub fn open_invoice_with_callback<F>(&self, url: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let cb = Closure::once_into_js(move |status: JsValue| {
            callback(status.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"openInvoice".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &url.into(), &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.openInvoice`. Resolves with the invoice
    /// status string (`paid`, `cancelled`, `failed`, `pending`).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn open_invoice(&self, url: &str) -> Result<String, JsValue> {
        let webapp = self.inner.clone();
        let url = url.to_owned();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |status: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &status);
            });
            Reflect::get(&webapp, &"openInvoice".into())?
                .dyn_into::<Function>()?
                .call2(&webapp, &url.into(), &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }

    /// Callback variant of [`Self::download_file`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails or the parameters
    /// fail to serialize.
    pub fn download_file_with_callback<F>(
        &self,
        params: DownloadFileParams<'_>,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let js_params =
            to_value(&params).map_err(|e| JsValue::from_str(&format!("serialize params: {e}")))?;
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"downloadFile".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &js_params, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.downloadFile`. Resolves with the file id
    /// string that Telegram returns.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails or the parameters
    /// fail to serialize.
    pub async fn download_file(&self, params: DownloadFileParams<'_>) -> Result<String, JsValue> {
        let js_params =
            to_value(&params).map_err(|e| JsValue::from_str(&format!("serialize params: {e}")))?;
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |v: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &v);
            });
            Reflect::get(&webapp, &"downloadFile".into())?
                .dyn_into::<Function>()?
                .call2(&webapp, &js_params, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }

    /// Callback variant of [`Self::read_text_from_clipboard`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn read_text_from_clipboard_with_callback<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let cb = Closure::once_into_js(move |text: JsValue| {
            callback(text.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"readTextFromClipboard".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("readTextFromClipboard is not a function"))?;
        func.call1(&self.inner, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.readTextFromClipboard`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn read_text_from_clipboard(&self) -> Result<String, JsValue> {
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |text: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &text);
            });
            let f = Reflect::get(&webapp, &"readTextFromClipboard".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("readTextFromClipboard is not a function"))?;
            func.call1(&webapp, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }
}
