// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::{
    logger,
    webapp::{
        TelegramWebApp,
        types::{
            BottomButton, BottomButtonParams, EventHandle, SecondaryButtonParams,
            SecondaryButtonPosition
        }
    }
};

impl TelegramWebApp {
    // === Internal bottom button helpers ===

    pub(super) fn bottom_button_object(&self, button: BottomButton) -> Result<Object, JsValue> {
        let name = button.js_name();
        Reflect::get(&self.inner, &name.into())
            .inspect_err(|_| logger::error(&format!("{name} not available")))?
            .dyn_into::<Object>()
            .inspect_err(|_| logger::error(&format!("{name} is not an object")))
    }

    pub(super) fn bottom_button_method(
        &self,
        button: BottomButton,
        method: &str,
        arg: Option<&JsValue>
    ) -> Result<(), JsValue> {
        let name = button.js_name();
        let btn = self.bottom_button_object(button)?;
        let f = Reflect::get(&btn, &method.into())
            .inspect_err(|_| logger::error(&format!("{name}.{method} not available")))?;
        let func = f.dyn_ref::<Function>().ok_or_else(|| {
            logger::error(&format!("{name}.{method} is not a function"));
            JsValue::from_str("not a function")
        })?;
        let result = match arg {
            Some(v) => func.call1(&btn, v),
            None => func.call0(&btn)
        };
        result.inspect_err(|_| logger::error(&format!("{name}.{method} call failed")))?;
        Ok(())
    }

    pub(super) fn bottom_button_property(
        &self,
        button: BottomButton,
        property: &str
    ) -> Option<JsValue> {
        self.bottom_button_object(button)
            .ok()
            .and_then(|object| Reflect::get(&object, &property.into()).ok())
    }

    // === Bottom button operations ===

    /// Call `WebApp.MainButton.show()` or `WebApp.SecondaryButton.show()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_bottom_button(&self, button: BottomButton) -> Result<(), JsValue> {
        self.bottom_button_method(button, "show", None)
    }

    /// Hide a bottom button.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn hide_bottom_button(&self, button: BottomButton) -> Result<(), JsValue> {
        self.bottom_button_method(button, "hide", None)
    }

    /// Set bottom button text.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_bottom_button_text(&self, button: BottomButton, text: &str) -> Result<(), JsValue> {
        self.bottom_button_method(button, "setText", Some(&text.into()))
    }

    /// Set bottom button color (`setColor(color)`).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::{TelegramWebApp, BottomButton};
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_bottom_button_color(BottomButton::Main, "#ff0000");
    /// ```
    pub fn set_bottom_button_color(
        &self,
        button: BottomButton,
        color: &str
    ) -> Result<(), JsValue> {
        self.bottom_button_method(button, "setColor", Some(&color.into()))
    }

    /// Set bottom button text color (`setTextColor(color)`).
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::{TelegramWebApp, BottomButton};
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_bottom_button_text_color(BottomButton::Main, "#ffffff");
    /// ```
    pub fn set_bottom_button_text_color(
        &self,
        button: BottomButton,
        color: &str
    ) -> Result<(), JsValue> {
        self.bottom_button_method(button, "setTextColor", Some(&color.into()))
    }

    /// Enable a bottom button, allowing user interaction.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.enable_bottom_button(BottomButton::Main);
    /// }
    /// ```
    pub fn enable_bottom_button(&self, button: BottomButton) -> Result<(), JsValue> {
        self.bottom_button_method(button, "enable", None)
    }

    /// Disable a bottom button, preventing user interaction.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.disable_bottom_button(BottomButton::Main);
    /// }
    /// ```
    pub fn disable_bottom_button(&self, button: BottomButton) -> Result<(), JsValue> {
        self.bottom_button_method(button, "disable", None)
    }

    /// Show the circular loading indicator on a bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.show_bottom_button_progress(BottomButton::Main, false);
    /// }
    /// ```
    pub fn show_bottom_button_progress(
        &self,
        button: BottomButton,
        leave_active: bool
    ) -> Result<(), JsValue> {
        let leave_active = JsValue::from_bool(leave_active);
        self.bottom_button_method(button, "showProgress", Some(&leave_active))
    }

    /// Hide the loading indicator on a bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.hide_bottom_button_progress(BottomButton::Main);
    /// }
    /// ```
    pub fn hide_bottom_button_progress(&self, button: BottomButton) -> Result<(), JsValue> {
        self.bottom_button_method(button, "hideProgress", None)
    }

    /// Returns whether the specified bottom button is currently visible.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_bottom_button_visible(BottomButton::Main);
    /// }
    /// ```
    pub fn is_bottom_button_visible(&self, button: BottomButton) -> bool {
        self.bottom_button_property(button, "isVisible")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns whether the specified bottom button is active (enabled).
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_bottom_button_active(BottomButton::Main);
    /// }
    /// ```
    pub fn is_bottom_button_active(&self, button: BottomButton) -> bool {
        self.bottom_button_property(button, "isActive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns whether the progress indicator is visible on the button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.is_bottom_button_progress_visible(BottomButton::Main);
    /// }
    /// ```
    pub fn is_bottom_button_progress_visible(&self, button: BottomButton) -> bool {
        self.bottom_button_property(button, "isProgressVisible")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Returns the current text displayed on the button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.bottom_button_text(BottomButton::Main);
    /// }
    /// ```
    pub fn bottom_button_text(&self, button: BottomButton) -> Option<String> {
        self.bottom_button_property(button, "text")?.as_string()
    }

    /// Returns the current text color of the button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.bottom_button_text_color(BottomButton::Main);
    /// }
    /// ```
    pub fn bottom_button_text_color(&self, button: BottomButton) -> Option<String> {
        self.bottom_button_property(button, "textColor")?
            .as_string()
    }

    /// Returns the current background color of the button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.bottom_button_color(BottomButton::Main);
    /// }
    /// ```
    pub fn bottom_button_color(&self, button: BottomButton) -> Option<String> {
        self.bottom_button_property(button, "color")?.as_string()
    }

    /// Returns whether the shine effect is enabled on the button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.bottom_button_has_shine_effect(BottomButton::Main);
    /// }
    /// ```
    pub fn bottom_button_has_shine_effect(&self, button: BottomButton) -> bool {
        self.bottom_button_property(button, "hasShineEffect")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Update bottom button state via `setParams`.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{BottomButton, BottomButtonParams, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let params = BottomButtonParams {
    ///         text: Some("Send"),
    ///         ..Default::default()
    ///     };
    ///     let _ = app.set_bottom_button_params(BottomButton::Main, &params);
    /// }
    /// ```
    pub fn set_bottom_button_params(
        &self,
        button: BottomButton,
        params: &BottomButtonParams<'_>
    ) -> Result<(), JsValue> {
        let value = to_value(params).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.bottom_button_method(button, "setParams", Some(&value))
    }

    /// Update secondary button state via `setParams`, including position.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{
    ///     SecondaryButtonParams, SecondaryButtonPosition, TelegramWebApp
    /// };
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let params = SecondaryButtonParams {
    ///         position: Some(SecondaryButtonPosition::Left),
    ///         ..Default::default()
    ///     };
    ///     let _ = app.set_secondary_button_params(&params);
    /// }
    /// ```
    pub fn set_secondary_button_params(
        &self,
        params: &SecondaryButtonParams<'_>
    ) -> Result<(), JsValue> {
        let value = to_value(params).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.bottom_button_method(BottomButton::Secondary, "setParams", Some(&value))
    }

    /// Returns the configured position of the secondary button, if available.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::{SecondaryButtonPosition, TelegramWebApp};
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.secondary_button_position();
    /// }
    /// ```
    pub fn secondary_button_position(&self) -> Option<SecondaryButtonPosition> {
        self.bottom_button_property(BottomButton::Secondary, "position")
            .and_then(SecondaryButtonPosition::from_js_value)
    }

    /// Set callback for `onClick()` on a bottom button.
    ///
    /// Returns an [`EventHandle`] that can be used to remove the callback.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_bottom_button_callback<F>(
        &self,
        button: BottomButton,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        let btn_val = Reflect::get(&self.inner, &button.js_name().into())?;
        let btn = btn_val.dyn_into::<Object>()?;
        let cb = Closure::<dyn FnMut()>::new(callback);
        let f = Reflect::get(&btn, &"onClick".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("onClick is not a function"))?;
        func.call1(&btn, cb.as_ref().unchecked_ref())?;
        Ok(EventHandle::new(btn, "offClick", None, cb))
    }

    /// Remove previously set bottom button callback.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn remove_bottom_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        handle.unregister()
    }

    // === Back button operations ===

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

    // === Legacy aliases for main button ===

    /// Legacy alias for [`Self::show_bottom_button`] with
    /// [`BottomButton::Main`].
    pub fn show_main_button(&self) -> Result<(), JsValue> {
        self.show_bottom_button(BottomButton::Main)
    }

    /// Legacy alias for [`Self::hide_bottom_button`] with
    /// [`BottomButton::Main`].
    pub fn hide_main_button(&self) -> Result<(), JsValue> {
        self.hide_bottom_button(BottomButton::Main)
    }

    /// Legacy alias for [`Self::set_bottom_button_text`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_text(&self, text: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text(BottomButton::Main, text)
    }

    /// Legacy alias for [`Self::set_bottom_button_color`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_color(BottomButton::Main, color)
    }

    /// Legacy alias for [`Self::set_bottom_button_text_color`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text_color(BottomButton::Main, color)
    }

    /// Enable the main bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.enable_main_button();
    /// }
    /// ```
    pub fn enable_main_button(&self) -> Result<(), JsValue> {
        self.enable_bottom_button(BottomButton::Main)
    }

    /// Disable the main bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.disable_main_button();
    /// }
    /// ```
    pub fn disable_main_button(&self) -> Result<(), JsValue> {
        self.disable_bottom_button(BottomButton::Main)
    }

    /// Show progress on the main bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.show_main_button_progress(false);
    /// }
    /// ```
    pub fn show_main_button_progress(&self, leave_active: bool) -> Result<(), JsValue> {
        self.show_bottom_button_progress(BottomButton::Main, leave_active)
    }

    /// Hide progress indicator from the main bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.hide_main_button_progress();
    /// }
    /// ```
    pub fn hide_main_button_progress(&self) -> Result<(), JsValue> {
        self.hide_bottom_button_progress(BottomButton::Main)
    }

    /// Update the main button state via
    /// [`set_bottom_button_params`](Self::set_bottom_button_params).
    pub fn set_main_button_params(&self, params: &BottomButtonParams<'_>) -> Result<(), JsValue> {
        self.set_bottom_button_params(BottomButton::Main, params)
    }

    /// Legacy alias for [`Self::set_bottom_button_callback`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_callback<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        self.set_bottom_button_callback(BottomButton::Main, callback)
    }

    /// Legacy alias for [`Self::remove_bottom_button_callback`].
    pub fn remove_main_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        self.remove_bottom_button_callback(handle)
    }

    // === Secondary button convenience methods ===

    /// Show the secondary bottom button.
    pub fn show_secondary_button(&self) -> Result<(), JsValue> {
        self.show_bottom_button(BottomButton::Secondary)
    }

    /// Hide the secondary bottom button.
    pub fn hide_secondary_button(&self) -> Result<(), JsValue> {
        self.hide_bottom_button(BottomButton::Secondary)
    }

    /// Set text for the secondary bottom button.
    pub fn set_secondary_button_text(&self, text: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text(BottomButton::Secondary, text)
    }

    /// Set color for the secondary bottom button.
    pub fn set_secondary_button_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_color(BottomButton::Secondary, color)
    }

    /// Set text color for the secondary bottom button.
    pub fn set_secondary_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text_color(BottomButton::Secondary, color)
    }

    /// Enable the secondary bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.enable_secondary_button();
    /// }
    /// ```
    pub fn enable_secondary_button(&self) -> Result<(), JsValue> {
        self.enable_bottom_button(BottomButton::Secondary)
    }

    /// Disable the secondary bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.disable_secondary_button();
    /// }
    /// ```
    pub fn disable_secondary_button(&self) -> Result<(), JsValue> {
        self.disable_bottom_button(BottomButton::Secondary)
    }

    /// Show progress on the secondary bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.show_secondary_button_progress(false);
    /// }
    /// ```
    pub fn show_secondary_button_progress(&self, leave_active: bool) -> Result<(), JsValue> {
        self.show_bottom_button_progress(BottomButton::Secondary, leave_active)
    }

    /// Hide progress indicator from the secondary bottom button.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::webapp::TelegramWebApp;
    ///
    /// if let Some(app) = TelegramWebApp::instance() {
    ///     let _ = app.hide_secondary_button_progress();
    /// }
    /// ```
    pub fn hide_secondary_button_progress(&self) -> Result<(), JsValue> {
        self.hide_bottom_button_progress(BottomButton::Secondary)
    }

    /// Set callback for the secondary bottom button.
    pub fn set_secondary_button_callback<F>(
        &self,
        callback: F
    ) -> Result<EventHandle<dyn FnMut()>, JsValue>
    where
        F: 'static + Fn()
    {
        self.set_bottom_button_callback(BottomButton::Secondary, callback)
    }

    /// Remove callback for the secondary bottom button.
    pub fn remove_secondary_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        self.remove_bottom_button_callback(handle)
    }

    /// Hide the on-screen keyboard.
    /// Call `WebApp.hideKeyboard()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.hide_keyboard().unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn hide_keyboard(&self) -> Result<(), JsValue> {
        self.call0("hideKeyboard")
    }
}
