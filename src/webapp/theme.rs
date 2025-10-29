// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::JsValue;

use crate::webapp::TelegramWebApp;

impl TelegramWebApp {
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
