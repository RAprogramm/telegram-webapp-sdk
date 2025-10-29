// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::webapp::{TelegramWebApp, types::SafeAreaInset};

impl TelegramWebApp {
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

    /// Call `WebApp.expand()` to expand the viewport.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn expand_viewport(&self) -> Result<(), JsValue> {
        self.call0("expand")
    }

    pub(super) fn safe_area_from_property(&self, property: &str) -> Option<SafeAreaInset> {
        let value = Reflect::get(&self.inner, &property.into()).ok()?;
        SafeAreaInset::from_js(value)
    }

    /// Returns the safe area insets reported by Telegram.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.safe_area_inset();
    /// }
    /// ```
    pub fn safe_area_inset(&self) -> Option<SafeAreaInset> {
        self.safe_area_from_property("safeAreaInset")
    }

    /// Returns the content safe area insets reported by Telegram.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.content_safe_area_inset();
    /// }
    /// ```
    pub fn content_safe_area_inset(&self) -> Option<SafeAreaInset> {
        self.safe_area_from_property("contentSafeAreaInset")
    }
}
