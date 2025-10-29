// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

use crate::{
    core::context::TelegramContext,
    validate_init_data::{self, ValidationKey},
    webapp::TelegramWebApp
};

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

    /// Try to get instance of `Telegram.WebApp`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the `Telegram.WebApp` object is missing or
    /// malformed.
    pub fn try_instance() -> Result<Self, JsValue> {
        let win = window().ok_or_else(|| JsValue::from_str("window not available"))?;
        let tg = Reflect::get(&win, &"Telegram".into())?;
        let webapp = Reflect::get(&tg, &"WebApp".into())?;
        let inner = webapp.dyn_into::<Object>()?;
        Ok(Self {
            inner
        })
    }

    /// Validate an `initData` payload using either HMAC-SHA256 or Ed25519.
    ///
    /// Pass [`ValidationKey::BotToken`] to verify the `hash` parameter using
    /// the bot token. Use [`ValidationKey::Ed25519PublicKey`] to verify the
    /// `signature` parameter with an Ed25519 public key.
    ///
    /// # Errors
    /// Returns [`validate_init_data::ValidationError`] if validation fails.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::{TelegramWebApp, validate_init_data::ValidationKey};
    /// let bot_token = "123456:ABC";
    /// let query = "a=1&b=2&hash=9e5e8d7c0b1f9f3a";
    /// TelegramWebApp::validate_init_data(query, ValidationKey::BotToken(bot_token)).unwrap();
    /// ```
    pub fn validate_init_data(
        init_data: &str,
        key: ValidationKey
    ) -> Result<(), validate_init_data::ValidationError> {
        match key {
            ValidationKey::BotToken(token) => {
                validate_init_data::verify_hmac_sha256(init_data, token)
            }
            ValidationKey::Ed25519PublicKey(pk) => {
                validate_init_data::verify_ed25519(init_data, pk)
            }
        }
    }

    /// Returns the raw initData string as provided by Telegram.
    ///
    /// This is the URL-encoded initData string captured during SDK
    /// initialization, suitable for server-side signature validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the SDK has not been initialized via
    /// [`crate::core::init::init_sdk`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telegram_webapp_sdk::TelegramWebApp;
    ///
    /// match TelegramWebApp::get_raw_init_data() {
    ///     Ok(raw) => {
    ///         // Send to backend for validation
    ///         println!("Raw initData: {}", raw);
    ///     }
    ///     Err(e) => eprintln!("SDK not initialized: {}", e)
    /// }
    /// ```
    pub fn get_raw_init_data() -> Result<String, &'static str> {
        TelegramContext::get_raw_init_data()
    }

    /// Call `WebApp.sendData(data)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn send_data(&self, data: &str) -> Result<(), JsValue> {
        self.call1("sendData", &data.into())
    }

    /// Returns whether the WebApp version is at least the provided value.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_version_at_least("9.0");
    /// }
    /// ```
    pub fn is_version_at_least(&self, version: &str) -> Result<bool, JsValue> {
        let f = Reflect::get(&self.inner, &"isVersionAtLeast".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("isVersionAtLeast is not a function"))?;
        let result = func.call1(&self.inner, &version.into())?;
        Ok(result.as_bool().unwrap_or(false))
    }

    /// Call `WebApp.ready()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn ready(&self) -> Result<(), JsValue> {
        self.call0("ready")
    }

    // === Internal helper methods ===

    pub(super) fn call0(&self, method: &str) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&self.inner)?;
        Ok(())
    }

    pub(super) fn call1(&self, method: &str, arg: &JsValue) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call1(&self.inner, arg)?;
        Ok(())
    }

    pub(super) fn call_nested0(&self, field: &str, method: &str) -> Result<(), JsValue> {
        let obj = Reflect::get(&self.inner, &field.into())?;
        let f = Reflect::get(&obj, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&obj)?;
        Ok(())
    }
}
