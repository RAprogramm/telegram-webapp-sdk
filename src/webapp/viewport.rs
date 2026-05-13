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

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use js_sys::{Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use crate::webapp::TelegramWebApp;

    wasm_bindgen_test_configure!(run_in_browser);

    fn setup_webapp() -> Object {
        let win = window().expect("window");
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_height_returns_f64() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportHeight".into(), &JsValue::from_f64(640.0));
        let app = TelegramWebApp::instance().expect("instance");
        assert_eq!(app.viewport_height(), Some(640.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_height_returns_none_when_property_missing() {
        let _ = setup_webapp();
        let app = TelegramWebApp::instance().expect("instance");
        assert!(app.viewport_height().is_none());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_width_reads_viewport_width_property() {
        // NOTE: real Telegram does not expose `viewportWidth`; this test pins
        // current crate behaviour so a future removal is intentional.
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportWidth".into(), &JsValue::from_f64(360.0));
        let app = TelegramWebApp::instance().expect("instance");
        assert_eq!(app.viewport_width(), Some(360.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_stable_height_returns_f64() {
        let webapp = setup_webapp();
        let _ = Reflect::set(
            &webapp,
            &"viewportStableHeight".into(),
            &JsValue::from_f64(600.0)
        );
        let app = TelegramWebApp::instance().expect("instance");
        assert_eq!(app.viewport_stable_height(), Some(600.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn expand_viewport_calls_js_expand() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_ref = called.clone();
        let cb = Closure::<dyn FnMut()>::new(move || called_ref.set(true));
        let _ = Reflect::set(&webapp, &"expand".into(), cb.as_ref().unchecked_ref());
        cb.forget();

        let app = TelegramWebApp::instance().expect("instance");
        app.expand_viewport().expect("ok");
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn expand_viewport_errors_when_method_missing() {
        let _ = setup_webapp();
        let app = TelegramWebApp::instance().expect("instance");
        assert!(app.expand_viewport().is_err());
    }
}
