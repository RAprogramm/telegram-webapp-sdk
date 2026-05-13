// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::webapp::{
    TelegramWebApp,
    core::{await_one_shot, one_shot_promise},
    types::OpenLinkOptions
};

impl TelegramWebApp {
    /// Call `WebApp.openLink(url)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_link("https://example.com", None).unwrap();
    /// ```
    pub fn open_link(&self, url: &str, options: Option<&OpenLinkOptions>) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"openLink".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("openLink is not a function"))?;
        match options {
            Some(opts) => {
                let value = to_value(opts).map_err(|err| JsValue::from_str(&err.to_string()))?;
                func.call2(&self.inner, &url.into(), &value)?;
            }
            None => {
                func.call1(&self.inner, &url.into())?;
            }
        }
        Ok(())
    }

    /// Call `WebApp.openTelegramLink(url)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_telegram_link("https://t.me/telegram").unwrap();
    /// ```
    pub fn open_telegram_link(&self, url: &str) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"openTelegramLink".into())?
            .dyn_into::<Function>()?
            .call1(&self.inner, &url.into())?;
        Ok(())
    }

    /// Call `WebApp.switchInlineQuery(query, choose_chat_types)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.switch_inline_query("query", None).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn switch_inline_query(
        &self,
        query: &str,
        choose_chat_types: Option<&JsValue>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"switchInlineQuery".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("switchInlineQuery is not a function"))?;
        match choose_chat_types {
            Some(types) => func.call2(&self.inner, &query.into(), types)?,
            None => func.call1(&self.inner, &query.into())?
        };
        Ok(())
    }

    /// Callback variant of [`Self::share_message`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_message_with_callback<F>(&self, msg_id: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"shareMessage".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareMessage is not a function"))?;
        func.call2(&self.inner, &msg_id.into(), &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.shareMessage`. Resolves with `true` when the
    /// prepared message was sent.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
    /// let app = TelegramWebApp::try_instance()?;
    /// let sent: bool = app.share_message("id123").await?;
    /// let _ = sent;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn share_message(&self, msg_id: &str) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let msg_id = msg_id.to_owned();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |v: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &v);
            });
            let f = Reflect::get(&webapp, &"shareMessage".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("shareMessage is not a function"))?;
            func.call2(&webapp, &msg_id.into(), &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Call `WebApp.shareToStory(media_url, params)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.share_to_story("https://example.com/image.png", Some(&params.into()))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_to_story(
        &self,
        media_url: &str,
        params: Option<&JsValue>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"shareToStory".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareToStory is not a function"))?;
        match params {
            Some(p) => func.call2(&self.inner, &media_url.into(), p)?,
            None => func.call1(&self.inner, &media_url.into())?
        };
        Ok(())
    }

    /// Call `WebApp.shareURL(url, text)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.share_url("https://example.com", Some("Check this"))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_url(&self, url: &str, text: Option<&str>) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"shareURL".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareURL is not a function"))?;
        match text {
            Some(t) => func.call2(&self.inner, &url.into(), &t.into())?,
            None => func.call1(&self.inner, &url.into())?
        };
        Ok(())
    }

    /// Callback variant of [`Self::request_chat`] (Bot API 9.6+).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_chat_with_callback<F>(&self, req_id: i32, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"requestChat".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("requestChat is not a function"))?;
        func.call2(&self.inner, &req_id.into(), &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.requestChat` (Bot API 9.6+). Resolves with
    /// `true` when the user picks a chat, `false` on cancel/failure.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # async fn run() -> Result<(), wasm_bindgen::JsValue> {
    /// let app = TelegramWebApp::try_instance()?;
    /// let sent: bool = app.request_chat(42).await?;
    /// let _ = sent;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails (including when the
    /// running Telegram client predates Bot API 9.6).
    pub async fn request_chat(&self, req_id: i32) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |v: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &v);
            });
            let f = Reflect::get(&webapp, &"requestChat".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("requestChat is not a function"))?;
            func.call2(&webapp, &req_id.into(), &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Call `WebApp.addToHomeScreen()` and return whether the prompt was shown.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _shown = app.add_to_home_screen().unwrap();
    /// ```
    pub fn add_to_home_screen(&self) -> Result<bool, JsValue> {
        let f = Reflect::get(&self.inner, &"addToHomeScreen".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("addToHomeScreen is not a function"))?;
        let result = func.call0(&self.inner)?;
        Ok(result.as_bool().unwrap_or(false))
    }

    /// Callback variant of [`Self::check_home_screen_status`].
    pub fn check_home_screen_status_with_callback<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let cb = Closure::once_into_js(move |status: JsValue| {
            callback(status.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"checkHomeScreenStatus".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("checkHomeScreenStatus is not a function"))?;
        func.call1(&self.inner, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.checkHomeScreenStatus`. Resolves with the
    /// status string Telegram returns (e.g. `"added"`, `"missed"`).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn check_home_screen_status(&self) -> Result<String, JsValue> {
        let webapp = self.inner.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |status: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &status);
            });
            let f = Reflect::get(&webapp, &"checkHomeScreenStatus".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("checkHomeScreenStatus is not a function"))?;
            func.call1(&webapp, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }
}
