// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::webapp::{
    TelegramWebApp,
    types::{BackgroundEvent, EventHandle}
};

impl TelegramWebApp {
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

    /// Register a callback for a background event.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_background_event<F>(
        &self,
        event: BackgroundEvent,
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
        func.call2(
            &self.inner,
            &event.as_str().into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some(event.as_str().to_string()),
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

    /// Register a callback for theme changes.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_theme_changed<F>(&self, callback: F) -> Result<EventHandle<dyn FnMut()>, JsValue>
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
            &"themeChanged".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("themeChanged".to_string()),
            cb
        ))
    }

    /// Register a callback for safe area changes.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_safe_area_changed<F>(&self, callback: F) -> Result<EventHandle<dyn FnMut()>, JsValue>
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
            &"safeAreaChanged".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("safeAreaChanged".to_string()),
            cb
        ))
    }

    /// Register a callback for content safe area changes.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_content_safe_area_changed<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
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
            &"contentSafeAreaChanged".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("contentSafeAreaChanged".to_string()),
            cb
        ))
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

    /// Register a callback for received clipboard text.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_clipboard_text_received<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut(JsValue)>, JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |text: JsValue| {
            callback(text.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"onEvent".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onEvent is not a function"))?;
        func.call2(
            &self.inner,
            &"clipboardTextReceived".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("clipboardTextReceived".to_string()),
            cb
        ))
    }

    /// Register a callback for invoice payment result.
    ///
    /// Returns an [`EventHandle`] that can be passed to
    /// [`off_event`](Self::off_event).
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let handle = app
    ///     .on_invoice_closed(|status| {
    ///         let _ = status;
    ///     })
    ///     .unwrap();
    /// app.off_event(handle).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn on_invoice_closed<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut(String)>, JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(String)>::new(callback);
        let f = Reflect::get(&self.inner, &"onEvent".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onEvent is not a function"))?;
        func.call2(
            &self.inner,
            &"invoiceClosed".into(),
            cb.as_ref().unchecked_ref()
        )?;
        Ok(EventHandle::new(
            self.inner.clone(),
            "offEvent",
            Some("invoiceClosed".to_string()),
            cb
        ))
    }
}
