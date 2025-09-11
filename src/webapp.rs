use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::window;

use crate::logger;

/// Handle returned when registering callbacks.
pub struct EventHandle<T: ?Sized> {
    target:   Object,
    method:   &'static str,
    event:    Option<String>,
    callback: Closure<T>
}

impl<T: ?Sized> EventHandle<T> {
    fn new(
        target: Object,
        method: &'static str,
        event: Option<String>,
        callback: Closure<T>
    ) -> Self {
        Self {
            target,
            method,
            event,
            callback
        }
    }

    pub(crate) fn unregister(self) -> Result<(), JsValue> {
        let f = Reflect::get(&self.target, &self.method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str(&format!("{} is not a function", self.method)))?;
        match self.event {
            Some(event) => func.call2(
                &self.target,
                &event.into(),
                self.callback.as_ref().unchecked_ref()
            )?,
            None => func.call1(&self.target, self.callback.as_ref().unchecked_ref())?
        };
        Ok(())
    }
}

/// Safe wrapper around `window.Telegram.WebApp`
#[derive(Clone)]
pub struct TelegramWebApp {
    inner: Object
}

impl TelegramWebApp {
    /// Get instance of `Telegram.WebApp` or `None` if not present
    pub fn instance() -> Option<Self> {
        let win = window()?;
        let tg = Reflect::get(&win, &"Telegram".into()).ok()?;
        let webapp = Reflect::get(&tg, &"WebApp".into()).ok()?;
        webapp.dyn_into::<Object>().ok().map(|inner| Self {
            inner
        })
    }

    /// Call `WebApp.sendData(data)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn send_data(&self, data: &str) -> Result<(), JsValue> {
        self.call1("sendData", &data.into())
    }

    /// Call `WebApp.expand()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn expand(&self) -> Result<(), JsValue> {
        self.call0("expand")
    }

    /// Call `WebApp.close()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn close(&self) -> Result<(), JsValue> {
        self.call0("close")
    }

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

    /// Call `WebApp.openLink(url)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_link("https://example.com").unwrap();
    /// ```
    pub fn open_link(&self, url: &str) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"openLink".into())?
            .dyn_into::<Function>()?
            .call1(&self.inner, &url.into())?;
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

    /// Call `WebApp.joinVoiceChat(chat_id, invite_hash)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.join_voice_chat("chat", None).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn join_voice_chat(
        &self,
        chat_id: &str,
        invite_hash: Option<&str>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"joinVoiceChat".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("joinVoiceChat is not a function"))?;
        match invite_hash {
            Some(hash) => func.call2(&self.inner, &chat_id.into(), &hash.into())?,
            None => func.call1(&self.inner, &chat_id.into())?
        };
        Ok(())
    }

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
        let f = Reflect::get(&self.inner, &"requestWriteAccess".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("requestWriteAccess is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
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

    /// Call `WebApp.MainButton.show()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_main_button(&self) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())?;
        let f = Reflect::get(&main_button, &"show".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("show is not a function"))?;
        func.call0(&main_button)?;
        Ok(())
    }

    /// Call `WebApp.MainButton.hide()`.
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.hide_main_button();
    /// ```
    pub fn hide_main_button(&self) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let hide = Reflect::get(&main_button, &"hide".into())
            .inspect_err(|_| logger::error("MainButton.hide not available"))?;
        let func = hide
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.hide is not a function"))?;
        func.call0(&main_button)
            .inspect_err(|_| logger::error("MainButton.hide call failed"))?;
        Ok(())
    }
    /// Call `WebApp.ready()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn ready(&self) -> Result<(), JsValue> {
        self.call0("ready")
    }

    /// Show back button.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_back_button(&self) -> Result<(), JsValue> {
        self.call_nested0("BackButton", "show")
    }

    /// Hide back button.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn hide_back_button(&self) -> Result<(), JsValue> {
        self.call_nested0("BackButton", "hide")
    }

    /// Set main button text.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_main_button_text(&self, text: &str) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())?;
        let f = Reflect::get(&main_button, &"setText".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("setText is not a function"))?;
        func.call1(&main_button, &text.into())?;
        Ok(())
    }

    /// Set main button color (`MainButton.setColor(color)`).
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_main_button_color("#ff0000");
    /// ```
    pub fn set_main_button_color(&self, color: &str) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let set_color = Reflect::get(&main_button, &"setColor".into())
            .inspect_err(|_| logger::error("MainButton.setColor not available"))?;
        let func = set_color
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.setColor is not a function"))?;
        func.call1(&main_button, &color.into())
            .inspect_err(|_| logger::error("MainButton.setColor call failed"))?;
        Ok(())
    }

    /// Set main button text color (`MainButton.setTextColor(color)`).
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_main_button_text_color("#ffffff");
    /// ```
    pub fn set_main_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let set_color = Reflect::get(&main_button, &"setTextColor".into())
            .inspect_err(|_| logger::error("MainButton.setTextColor not available"))?;
        let func = set_color
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.setTextColor is not a function"))?;
        func.call1(&main_button, &color.into())
            .inspect_err(|_| logger::error("MainButton.setTextColor call failed"))?;
        Ok(())
    }

    /// Set callback for `MainButton.onClick()`.
    ///
    /// Returns an [`EventHandle`] that can be used to remove the callback.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_main_button_callback<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        let main_button_val = Reflect::get(&self.inner, &"MainButton".into())?;
        let main_button = main_button_val.dyn_into::<Object>()?;
        let cb = Closure::<dyn FnMut()>::new(callback);
        let f = Reflect::get(&main_button, &"onClick".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onClick is not a function"))?;
        func.call1(&main_button, cb.as_ref().unchecked_ref())?;
        Ok(EventHandle::new(main_button, "offClick", None, cb))
    }

    /// Remove previously set main button callback.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn remove_main_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        handle.unregister()
    }

    /// Register event handler (`web_app_event_name`, callback).
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_event<F>(
        &self,
        event: &str,
        callback: F
    ) -> Result<EventHandle<dyn FnMut(JsValue)>, JsValue>
    where
        F: 'static + Fn(JsValue)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(callback);
        let f = Reflect::get(&self.inner, &"onEvent".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onEvent is not a function"))?;
        func.call2(&self.inner, &event.into(), cb.as_ref().unchecked_ref())?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some(event.to_owned()),
            cb
        ))
    }

    /// Deregister a previously registered event handler.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn off_event<T: ?Sized>(&self, handle: EventHandle<T>) -> Result<(), JsValue> {
        handle.unregister()
    }

    /// Internal: call `this[field][method]()`
    fn call_nested0(&self, field: &str, method: &str) -> Result<(), JsValue> {
        let obj = Reflect::get(&self.inner, &field.into())?;
        let f = Reflect::get(&obj, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&obj)?;
        Ok(())
    }

    // === Internal generic method helpers ===

    fn call0(&self, method: &str) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&self.inner)?;
        Ok(())
    }

    fn call1(&self, method: &str, arg: &JsValue) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call1(&self.inner, arg)?;
        Ok(())
    }

    /// Returns the current viewport height in pixels.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.viewport_height();
    /// ```
    pub fn viewport_height(&self) -> Option<f64> {
        Reflect::get(&self.inner, &"viewportHeight".into())
            .ok()?
            .as_f64()
    }

    /// Returns the current viewport width in pixels.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.viewport_width();
    /// ```
    pub fn viewport_width(&self) -> Option<f64> {
        Reflect::get(&self.inner, &"viewportWidth".into())
            .ok()?
            .as_f64()
    }

    /// Returns the stable viewport height in pixels.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.viewport_stable_height();
    /// ```
    pub fn viewport_stable_height(&self) -> Option<f64> {
        Reflect::get(&self.inner, &"viewportStableHeight".into())
            .ok()?
            .as_f64()
    }

    pub fn is_expanded(&self) -> bool {
        Reflect::get(&self.inner, &"isExpanded".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Call `WebApp.expand()` to expand the viewport.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn expand_viewport(&self) -> Result<(), JsValue> {
        self.call0("expand")
    }

    /// Register a callback for viewport changes.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_viewport_changed<F>(&self, callback: F) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        let cb = Closure::<dyn FnMut()>::new(callback);
        let f = Reflect::get(&self.inner, &"onEvent".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onEvent is not a function"))?;
        func.call2(
            &self.inner,
            &"viewportChanged".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("viewportChanged".to_string()),
            cb
        ))
    }

    /// Registers a callback for the native back button.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`remove_back_button_callback`](Self::remove_back_button_callback).
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let handle = app.set_back_button_callback(|| {}).expect("callback");
    /// app.remove_back_button_callback(handle).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_back_button_callback<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        let back_button_val = Reflect::get(&self.inner, &"BackButton".into())?;
        let back_button = back_button_val.dyn_into::<Object>()?;
        let cb = Closure::<dyn FnMut()>::new(callback);
        let f = Reflect::get(&back_button, &"onClick".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onClick is not a function"))?;
        func.call1(&back_button, cb.as_ref().unchecked_ref())?;
        Ok(EventHandle::new(back_button, "offClick", None, cb))
    }

    /// Remove previously set back button callback.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn remove_back_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        handle.unregister()
    }
    /// Returns whether the native back button is visible.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.is_back_button_visible();
    /// ```
    pub fn is_back_button_visible(&self) -> bool {
        Reflect::get(&self.inner, &"BackButton".into())
            .ok()
            .and_then(|bb| Reflect::get(&bb, &"isVisible".into()).ok())
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc
    };

    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_main_button_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &main_button,
            &"hide".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.hide_main_button().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_main_button_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_main_button_color("#00ff00").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_main_button_text_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setTextColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_main_button_text_color("#112233").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_dimensions() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportWidth".into(), &JsValue::from_f64(320.0));
        let _ = Reflect::set(
            &webapp,
            &"viewportStableHeight".into(),
            &JsValue::from_f64(480.0)
        );
        let app = TelegramWebApp::instance().unwrap();
        assert_eq!(app.viewport_width(), Some(320.0));
        assert_eq!(app.viewport_stable_height(), Some(480.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn back_button_visibility_and_callback() {
        let webapp = setup_webapp();
        let back_button = Object::new();
        let _ = Reflect::set(&webapp, &"BackButton".into(), &back_button);
        let _ = Reflect::set(&back_button, &"isVisible".into(), &JsValue::TRUE);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&back_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&back_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_back_button_visible());
        let handle = app
            .set_back_button_callback(move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&back_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_back_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn main_button_callback_register_and_remove() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&main_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&main_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .set_main_button_callback(move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&main_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_main_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn on_event_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_event("test", |_: JsValue| {}).unwrap();
        assert!(Reflect::has(&webapp, &"test".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"test".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_viewport_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_link_and_telegram_link() {
        let webapp = setup_webapp();
        let open_link = Function::new_with_args("url", "this.open_link = url;");
        let open_tg_link = Function::new_with_args("url", "this.open_tg_link = url;");
        let _ = Reflect::set(&webapp, &"openLink".into(), &open_link);
        let _ = Reflect::set(&webapp, &"openTelegramLink".into(), &open_tg_link);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        app.open_link(url).unwrap();
        app.open_telegram_link(url).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"open_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
        assert_eq!(
            Reflect::get(&webapp, &"open_tg_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_invoice_invokes_callback() {
        let webapp = setup_webapp();
        let open_invoice = Function::new_with_args("url, cb", "cb('paid');");
        let _ = Reflect::set(&webapp, &"openInvoice".into(), &open_invoice);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);

        app.open_invoice("https://invoice", move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        assert_eq!(status.borrow().as_str(), "paid");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn switch_inline_query_calls_js() {
        let webapp = setup_webapp();
        let switch_inline =
            Function::new_with_args("query, types", "this.query = query; this.types = types;");
        let _ = Reflect::set(&webapp, &"switchInlineQuery".into(), &switch_inline);

        let app = TelegramWebApp::instance().unwrap();
        let types = JsValue::from_str("users");
        app.switch_inline_query("search", Some(&types)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"query".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("search"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"types".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("users"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_url_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args(
            "url, text",
            "this.shared_url = url; this.shared_text = text;"
        );
        let _ = Reflect::set(&webapp, &"shareURL".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        let text = "check";
        app.share_url(url, Some(text)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"shared_url".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url),
        );
        assert_eq!(
            Reflect::get(&webapp, &"shared_text".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(text),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn join_voice_chat_calls_js() {
        let webapp = setup_webapp();
        let join = Function::new_with_args(
            "id, hash",
            "this.voice_chat_id = id; this.voice_chat_hash = hash;"
        );
        let _ = Reflect::set(&webapp, &"joinVoiceChat".into(), &join);

        let app = TelegramWebApp::instance().unwrap();
        app.join_voice_chat("123", Some("hash")).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_id".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("123"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_hash".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("hash"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_write_access_invokes_callback() {
        let webapp = setup_webapp();
        let request = Function::new_with_args("cb", "cb(true);");
        let _ = Reflect::set(&webapp, &"requestWriteAccess".into(), &request);

        let app = TelegramWebApp::instance().unwrap();
        let granted = Rc::new(Cell::new(false));
        let granted_clone = Rc::clone(&granted);

        app.request_write_access(move |g| {
            granted_clone.set(g);
        })
        .unwrap();

        assert!(granted.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_popup_invokes_callback() {
        let webapp = setup_webapp();
        let show_popup = Function::new_with_args("params, cb", "cb('ok');");
        let _ = Reflect::set(&webapp, &"showPopup".into(), &show_popup);

        let app = TelegramWebApp::instance().unwrap();
        let button = Rc::new(RefCell::new(String::new()));
        let button_clone = Rc::clone(&button);

        app.show_popup(&JsValue::NULL, move |id| {
            *button_clone.borrow_mut() = id;
        })
        .unwrap();

        assert_eq!(button.borrow().as_str(), "ok");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn scan_qr_popup_invokes_callback_and_close() {
        let webapp = setup_webapp();
        let show_scan = Function::new_with_args("text, cb", "cb('code');");
        let close_scan = Function::new_with_args("", "this.closed = true;");
        let _ = Reflect::set(&webapp, &"showScanQrPopup".into(), &show_scan);
        let _ = Reflect::set(&webapp, &"closeScanQrPopup".into(), &close_scan);

        let app = TelegramWebApp::instance().unwrap();
        let text = Rc::new(RefCell::new(String::new()));
        let text_clone = Rc::clone(&text);

        app.show_scan_qr_popup("scan", move |value| {
            *text_clone.borrow_mut() = value;
        })
        .unwrap();
        assert_eq!(text.borrow().as_str(), "code");

        app.close_scan_qr_popup().unwrap();
        let closed = Reflect::get(&webapp, &"closed".into())
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(closed);
    }
}
