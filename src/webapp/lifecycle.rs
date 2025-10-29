// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::webapp::TelegramWebApp;

impl TelegramWebApp {
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

    /// Call `WebApp.enableClosingConfirmation()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.enable_closing_confirmation().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn enable_closing_confirmation(&self) -> Result<(), JsValue> {
        self.call0("enableClosingConfirmation")
    }

    /// Call `WebApp.disableClosingConfirmation()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.disable_closing_confirmation().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn disable_closing_confirmation(&self) -> Result<(), JsValue> {
        self.call0("disableClosingConfirmation")
    }

    /// Returns whether closing confirmation is currently enabled.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.is_closing_confirmation_enabled();
    /// ```
    pub fn is_closing_confirmation_enabled(&self) -> bool {
        Reflect::get(&self.inner, &"isClosingConfirmationEnabled".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Call `WebApp.requestFullscreen()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.request_fullscreen().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_fullscreen(&self) -> Result<(), JsValue> {
        self.call0("requestFullscreen")
    }

    /// Call `WebApp.exitFullscreen()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.exit_fullscreen().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn exit_fullscreen(&self) -> Result<(), JsValue> {
        self.call0("exitFullscreen")
    }

    /// Returns whether the app is displayed in fullscreen mode.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_fullscreen();
    /// }
    /// ```
    pub fn is_fullscreen(&self) -> bool {
        Reflect::get(&self.inner, &"isFullscreen".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Call `WebApp.lockOrientation(orientation)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.lock_orientation("portrait").unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn lock_orientation(&self, orientation: &str) -> Result<(), JsValue> {
        self.call1("lockOrientation", &orientation.into())
    }

    /// Call `WebApp.unlockOrientation()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.unlock_orientation().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn unlock_orientation(&self) -> Result<(), JsValue> {
        self.call0("unlockOrientation")
    }

    /// Returns whether the orientation is locked.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_orientation_locked();
    /// }
    /// ```
    pub fn is_orientation_locked(&self) -> bool {
        Reflect::get(&self.inner, &"isOrientationLocked".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Call `WebApp.enableVerticalSwipes()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.enable_vertical_swipes().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn enable_vertical_swipes(&self) -> Result<(), JsValue> {
        self.call0("enableVerticalSwipes")
    }

    /// Call `WebApp.disableVerticalSwipes()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.disable_vertical_swipes().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn disable_vertical_swipes(&self) -> Result<(), JsValue> {
        self.call0("disableVerticalSwipes")
    }

    /// Returns whether vertical swipes are currently enabled.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_vertical_swipes_enabled();
    /// }
    /// ```
    pub fn is_vertical_swipes_enabled(&self) -> bool {
        Reflect::get(&self.inner, &"isVerticalSwipesEnabled".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns whether the mini app is currently active (visible to the user).
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_active();
    /// }
    /// ```
    pub fn is_active(&self) -> bool {
        Reflect::get(&self.inner, &"isActive".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    pub fn is_expanded(&self) -> bool {
        Reflect::get(&self.inner, &"isExpanded".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}
