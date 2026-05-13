// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::webapp::TelegramWebApp;

impl TelegramWebApp {
    /// Returns `WebApp.colorScheme` — `"light"` or `"dark"`.
    pub fn color_scheme(&self) -> Option<String> {
        Reflect::get(&self.inner, &"colorScheme".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Returns the current `WebApp.headerColor` value.
    pub fn header_color(&self) -> Option<String> {
        Reflect::get(&self.inner, &"headerColor".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Returns the current `WebApp.backgroundColor` value.
    pub fn background_color(&self) -> Option<String> {
        Reflect::get(&self.inner, &"backgroundColor".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Returns the current `WebApp.bottomBarColor` value (Bot API 7.10+).
    pub fn bottom_bar_color(&self) -> Option<String> {
        Reflect::get(&self.inner, &"bottomBarColor".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Returns the raw `WebApp.version` string (e.g. `"9.6"`).
    pub fn raw_version(&self) -> Option<String> {
        Reflect::get(&self.inner, &"version".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Returns the `WebApp.platform` string (e.g. `"tdesktop"`, `"ios"`,
    /// `"web"`).
    pub fn platform(&self) -> Option<String> {
        Reflect::get(&self.inner, &"platform".into())
            .ok()
            .and_then(|v| v.as_string())
    }

    /// Call `WebApp.setHeaderColor(color)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.set_header_color("#ffffff").unwrap();
    /// ```
    pub fn set_header_color(&self, color: &str) -> Result<(), JsValue> {
        self.call1("setHeaderColor", &color.into())
    }

    /// Call `WebApp.setBackgroundColor(color)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.set_background_color("#ffffff").unwrap();
    /// ```
    pub fn set_background_color(&self, color: &str) -> Result<(), JsValue> {
        self.call1("setBackgroundColor", &color.into())
    }

    /// Call `WebApp.setBottomBarColor(color)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.set_bottom_bar_color("#ffffff").unwrap();
    /// ```
    pub fn set_bottom_bar_color(&self, color: &str) -> Result<(), JsValue> {
        self.call1("setBottomBarColor", &color.into())
    }
}

#[cfg(test)]
mod tests {
    use js_sys::{Object, Reflect};
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
    fn property_getters_read_string_values() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"colorScheme".into(), &"dark".into());
        let _ = Reflect::set(&webapp, &"headerColor".into(), &"#111111".into());
        let _ = Reflect::set(&webapp, &"backgroundColor".into(), &"#222222".into());
        let _ = Reflect::set(&webapp, &"bottomBarColor".into(), &"#333333".into());
        let _ = Reflect::set(&webapp, &"version".into(), &"9.6".into());
        let _ = Reflect::set(&webapp, &"platform".into(), &"tdesktop".into());

        let app = TelegramWebApp::instance().expect("instance");
        assert_eq!(app.color_scheme().as_deref(), Some("dark"));
        assert_eq!(app.header_color().as_deref(), Some("#111111"));
        assert_eq!(app.background_color().as_deref(), Some("#222222"));
        assert_eq!(app.bottom_bar_color().as_deref(), Some("#333333"));
        assert_eq!(app.raw_version().as_deref(), Some("9.6"));
        assert_eq!(app.platform().as_deref(), Some("tdesktop"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn property_getters_return_none_when_absent() {
        let _webapp = setup_webapp();
        let app = TelegramWebApp::instance().expect("instance");
        assert!(app.color_scheme().is_none());
        assert!(app.platform().is_none());
    }
}
