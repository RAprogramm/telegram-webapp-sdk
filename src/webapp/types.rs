// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

/// Handle returned when registering callbacks.
pub struct EventHandle<T: ?Sized> {
    pub(super) target:   Object,
    pub(super) method:   &'static str,
    pub(super) event:    Option<String>,
    pub(super) callback: Closure<T>
}

impl<T: ?Sized> EventHandle<T> {
    pub(super) fn new(
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

/// Identifies which bottom button to operate on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BottomButton {
    /// Primary bottom button.
    Main,
    /// Secondary bottom button.
    Secondary
}

impl BottomButton {
    pub(super) const fn js_name(self) -> &'static str {
        match self {
            BottomButton::Main => "MainButton",
            BottomButton::Secondary => "SecondaryButton"
        }
    }
}

/// Position of the secondary bottom button.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::webapp::{SecondaryButtonPosition, TelegramWebApp};
///
/// if let Some(app) = TelegramWebApp::instance() {
///     match app.secondary_button_position() {
///         Some(SecondaryButtonPosition::Top) => {}
///         _ => {}
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SecondaryButtonPosition {
    /// Displayed above the main button.
    Top,
    /// Displayed to the left of the main button.
    Left,
    /// Displayed below the main button.
    Bottom,
    /// Displayed to the right of the main button.
    Right
}

impl SecondaryButtonPosition {
    pub(super) fn from_js_value(value: JsValue) -> Option<Self> {
        let as_str = value.as_string()?;
        match as_str.as_str() {
            "top" => Some(Self::Top),
            "left" => Some(Self::Left),
            "bottom" => Some(Self::Bottom),
            "right" => Some(Self::Right),
            _ => None
        }
    }
}

/// Safe area insets reported by Telegram.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::webapp::{SafeAreaInset, TelegramWebApp};
///
/// if let Some(app) = TelegramWebApp::instance() {
///     if let Some(SafeAreaInset {
///         top,
///         bottom,
///         ..
///     }) = app.safe_area_inset()
///     {
///         let _ = (top, bottom);
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SafeAreaInset {
    /// Distance from the top edge in CSS pixels.
    pub top:    f64,
    /// Distance from the bottom edge in CSS pixels.
    pub bottom: f64,
    /// Distance from the left edge in CSS pixels.
    pub left:   f64,
    /// Distance from the right edge in CSS pixels.
    pub right:  f64
}

impl SafeAreaInset {
    pub(super) fn from_js(value: JsValue) -> Option<Self> {
        let object = value.dyn_into::<Object>().ok()?;
        let top = Reflect::get(&object, &"top".into()).ok()?.as_f64()?;
        let bottom = Reflect::get(&object, &"bottom".into()).ok()?.as_f64()?;
        let left = Reflect::get(&object, &"left".into()).ok()?.as_f64()?;
        let right = Reflect::get(&object, &"right".into()).ok()?.as_f64()?;
        Some(Self {
            top,
            bottom,
            left,
            right
        })
    }
}

/// Parameters accepted by bottom buttons when updating state via `setParams`.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::webapp::{BottomButton, BottomButtonParams, TelegramWebApp};
///
/// if let Some(app) = TelegramWebApp::instance() {
///     let params = BottomButtonParams {
///         text: Some("Send"),
///         is_active: Some(true),
///         ..Default::default()
///     };
///     let _ = app.set_bottom_button_params(BottomButton::Main, &params);
/// }
/// ```
#[derive(Debug, Default, Serialize)]
pub struct BottomButtonParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text:             Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color:            Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color:       Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active:        Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible:       Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_shine_effect: Option<bool>
}

/// Additional parameters supported by the secondary button.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::webapp::{
///     SecondaryButtonParams, SecondaryButtonPosition, TelegramWebApp
/// };
///
/// if let Some(app) = TelegramWebApp::instance() {
///     let params = SecondaryButtonParams {
///         common:   Default::default(),
///         position: Some(SecondaryButtonPosition::Top)
///     };
///     let _ = app.set_secondary_button_params(&params);
/// }
/// ```
#[derive(Debug, Default, Serialize)]
pub struct SecondaryButtonParams<'a> {
    #[serde(flatten)]
    pub common:   BottomButtonParams<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<SecondaryButtonPosition>
}

/// Options supported by [`crate::webapp::TelegramWebApp::open_link`].
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::webapp::{OpenLinkOptions, TelegramWebApp};
///
/// if let Some(app) = TelegramWebApp::instance() {
///     let options = OpenLinkOptions {
///         try_instant_view: Some(true)
///     };
///     let _ = app.open_link("https://example.com", Some(&options));
/// }
/// ```
#[derive(Debug, Default, Serialize)]
pub struct OpenLinkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_instant_view: Option<bool>
}

/// Background events delivered by Telegram when the Mini App runs in the
/// background.
#[derive(Clone, Copy, Debug)]
pub enum BackgroundEvent {
    /// The main button was clicked. Payload: [`JsValue::UNDEFINED`].
    MainButtonClicked,
    /// The back button was clicked. Payload: [`JsValue::UNDEFINED`].
    BackButtonClicked,
    /// The settings button was clicked. Payload: [`JsValue::UNDEFINED`].
    SettingsButtonClicked,
    /// User responded to a write access request. Payload: `bool`.
    WriteAccessRequested,
    /// User responded to a contact request. Payload: `bool`.
    ContactRequested,
    /// User responded to a phone number request. Payload: `bool`.
    PhoneRequested,
    /// An invoice was closed. Payload: status string.
    InvoiceClosed,
    /// A popup was closed. Payload: object containing `button_id`.
    PopupClosed,
    /// Text was received from the QR scanner. Payload: scanned text.
    QrTextReceived,
    /// Text was read from the clipboard. Payload: clipboard text.
    ClipboardTextReceived
}

impl BackgroundEvent {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            BackgroundEvent::MainButtonClicked => "mainButtonClicked",
            BackgroundEvent::BackButtonClicked => "backButtonClicked",
            BackgroundEvent::SettingsButtonClicked => "settingsButtonClicked",
            BackgroundEvent::WriteAccessRequested => "writeAccessRequested",
            BackgroundEvent::ContactRequested => "contactRequested",
            BackgroundEvent::PhoneRequested => "phoneRequested",
            BackgroundEvent::InvoiceClosed => "invoiceClosed",
            BackgroundEvent::PopupClosed => "popupClosed",
            BackgroundEvent::QrTextReceived => "qrTextReceived",
            BackgroundEvent::ClipboardTextReceived => "clipboardTextReceived"
        }
    }
}
