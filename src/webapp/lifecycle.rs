// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue};

use crate::webapp::{TelegramWebApp, types::CloseOptions};

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

    /// Call `WebApp.close(options)` (Bot API 7.6+ for `return_back`).
    ///
    /// On older Telegram clients the option is silently ignored on the JS side.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::{CloseOptions, TelegramWebApp};
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.close_with_options(&CloseOptions {
    ///     return_back: Some(true)
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails or the options fail
    /// to serialize.
    pub fn close_with_options(&self, options: &CloseOptions) -> Result<(), JsValue> {
        let payload = to_value(options).map_err(|err| JsValue::from_str(&err.to_string()))?;
        let f = Reflect::get(&self.inner, &"close".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("close is not a function"))?;
        func.call1(&self.inner, &payload)?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use crate::webapp::{TelegramWebApp, types::CloseOptions};

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
    fn close_with_options_passes_return_back() {
        let webapp = setup_webapp();
        let capture = Function::new_with_args("opts", "this.captured_close = opts;");
        let _ = Reflect::set(&webapp, &"close".into(), &capture);

        let app = TelegramWebApp::instance().expect("instance");
        app.close_with_options(&CloseOptions {
            return_back: Some(true)
        })
        .expect("ok");

        let opts = Reflect::get(&webapp, &"captured_close".into()).expect("captured");
        let val = Reflect::get(&opts, &"return_back".into()).expect("field");
        assert_eq!(val.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn close_with_options_omits_unset_fields() {
        let webapp = setup_webapp();
        let capture = Function::new_with_args("opts", "this.captured_close = opts;");
        let _ = Reflect::set(&webapp, &"close".into(), &capture);

        let app = TelegramWebApp::instance().expect("instance");
        app.close_with_options(&CloseOptions::default())
            .expect("ok");

        let opts = Reflect::get(&webapp, &"captured_close".into()).expect("captured");
        let val = Reflect::get(&opts, &"return_back".into()).expect("field");
        assert!(val.is_undefined());
    }
}
