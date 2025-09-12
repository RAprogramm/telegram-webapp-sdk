use js_sys::{Function, Object, Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::window;

use crate::{
    core::types::download_file_params::DownloadFileParams,
    logger,
    validate_init_data::{self, ValidationKey}
};

/// Handle returned when registering callbacks.
pub struct EventHandle<T: ?Sized> {
    target:   Object,
    method:   &'static str,
    event:    Option<String>,
    callback: Closure<T>
}

impl<T: ?Sized> EventHandle<T> {
    fn new(
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
#[derive(Clone, Copy, Debug)]
pub enum BottomButton {
    /// Primary bottom button.
    Main,
    /// Secondary bottom button.
    Secondary
}

impl BottomButton {
    const fn js_name(self) -> &'static str {
        match self {
            BottomButton::Main => "MainButton",
            BottomButton::Secondary => "SecondaryButton"
        }
    }
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
    const fn as_str(self) -> &'static str {
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

/// Safe wrapper around `window.Telegram.WebApp`
#[derive(Clone)]
pub struct TelegramWebApp {
    inner: Object
}

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

    /// Call `WebApp.sendData(data)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn send_data(&self, data: &str) -> Result<(), JsValue> {
        self.call1("sendData", &data.into())
    }

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

    /// Call `WebApp.showAlert(message)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_alert(&self, msg: &str) -> Result<(), JsValue> {
        self.call1("showAlert", &msg.into())
    }

    /// Call `WebApp.showConfirm(message, callback)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_confirm<F>(&self, msg: &str, on_confirm: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(bool)>::new(on_confirm);
        let f = Reflect::get(&self.inner, &"showConfirm".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("showConfirm is not a function"))?;
        func.call2(&self.inner, &msg.into(), cb.as_ref().unchecked_ref())?;
        cb.forget(); // safe leak for JS lifetime
        Ok(())
    }

    /// Call `WebApp.openLink(url)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_link("https://example.com").unwrap();
    /// ```
    pub fn open_link(&self, url: &str) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"openLink".into())?
            .dyn_into::<Function>()?
            .call1(&self.inner, &url.into())?;
        Ok(())
    }

    /// Call `WebApp.openTelegramLink(url)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_telegram_link("https://t.me/telegram").unwrap();
    /// ```
    pub fn open_telegram_link(&self, url: &str) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"openTelegramLink".into())?
            .dyn_into::<Function>()?
            .call1(&self.inner, &url.into())?;
        Ok(())
    }

    /// Call `WebApp.openInvoice(url, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.open_invoice("https://invoice", |status| {
    ///     let _ = status;
    /// })
    /// .unwrap();
    /// ```
    pub fn open_invoice<F>(&self, url: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |status: JsValue| {
            callback(status.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"openInvoice".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &url.into(), cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.switchInlineQuery(query, choose_chat_types)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.switch_inline_query("query", None).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn switch_inline_query(
        &self,
        query: &str,
        choose_chat_types: Option<&JsValue>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"switchInlineQuery".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("switchInlineQuery is not a function"))?;
        match choose_chat_types {
            Some(types) => func.call2(&self.inner, &query.into(), types)?,
            None => func.call1(&self.inner, &query.into())?
        };
        Ok(())
    }

    /// Call `WebApp.shareMessage(msg_id, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.share_message("id123", |sent| {
    ///     let _ = sent;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_message<F>(&self, msg_id: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"shareMessage".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareMessage is not a function"))?;
        func.call2(&self.inner, &msg_id.into(), cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.shareToStory(media_url, params)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.share_to_story("https://example.com/image.png", Some(&params.into()))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_to_story(
        &self,
        media_url: &str,
        params: Option<&JsValue>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"shareToStory".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareToStory is not a function"))?;
        match params {
            Some(p) => func.call2(&self.inner, &media_url.into(), p)?,
            None => func.call1(&self.inner, &media_url.into())?
        };
        Ok(())
    }

    /// Call `WebApp.shareURL(url, text)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.share_url("https://example.com", Some("Check this"))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn share_url(&self, url: &str, text: Option<&str>) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"shareURL".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("shareURL is not a function"))?;
        match text {
            Some(t) => func.call2(&self.inner, &url.into(), &t.into())?,
            None => func.call1(&self.inner, &url.into())?
        };
        Ok(())
    }

    /// Call `WebApp.joinVoiceChat(chat_id, invite_hash)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.join_voice_chat("chat", None).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn join_voice_chat(
        &self,
        chat_id: &str,
        invite_hash: Option<&str>
    ) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &"joinVoiceChat".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("joinVoiceChat is not a function"))?;
        match invite_hash {
            Some(hash) => func.call2(&self.inner, &chat_id.into(), &hash.into())?,
            None => func.call1(&self.inner, &chat_id.into())?
        };
        Ok(())
    }

    /// Call `WebApp.addToHomeScreen()` and return whether the prompt was shown.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _shown = app.add_to_home_screen().unwrap();
    /// ```
    pub fn add_to_home_screen(&self) -> Result<bool, JsValue> {
        let f = Reflect::get(&self.inner, &"addToHomeScreen".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("addToHomeScreen is not a function"))?;
        let result = func.call0(&self.inner)?;
        Ok(result.as_bool().unwrap_or(false))
    }

    /// Call `WebApp.checkHomeScreenStatus(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.check_home_screen_status(|status| {
    ///     let _ = status;
    /// })
    /// .unwrap();
    /// ```
    pub fn check_home_screen_status<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |status: JsValue| {
            callback(status.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"checkHomeScreenStatus".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("checkHomeScreenStatus is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.requestWriteAccess(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.request_write_access(|granted| {
    ///     let _ = granted;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_write_access<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        self.call1("requestWriteAccess", cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.downloadFile(params, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::core::types::download_file_params::DownloadFileParams;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = DownloadFileParams {
    ///     url:       "https://example.com/file",
    ///     file_name: None,
    ///     mime_type: None
    /// };
    /// app.download_file(params, |file_id| {
    ///     let _ = file_id;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails or the parameters
    /// fail to serialize.
    pub fn download_file<F>(
        &self,
        params: DownloadFileParams<'_>,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let js_params =
            to_value(&params).map_err(|e| JsValue::from_str(&format!("serialize params: {e}")))?;
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"downloadFile".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &js_params, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.requestEmojiStatusAccess(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.request_emoji_status_access(|granted| {
    ///     let _ = granted;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn request_emoji_status_access<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"requestEmojiStatusAccess".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("requestEmojiStatusAccess is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.setEmojiStatus(status, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use js_sys::Reflect;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let status = Object::new();
    /// let _ = Reflect::set(&status, &"custom_emoji_id".into(), &"123".into());
    /// app.set_emoji_status(&status.into(), |success| {
    ///     let _ = success;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn set_emoji_status<F>(&self, status: &JsValue, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            callback(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"setEmojiStatus".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("setEmojiStatus is not a function"))?;
        func.call2(&self.inner, status, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.showPopup(params, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.show_popup(&params.into(), |id| {
    ///     let _ = id;
    /// })
    /// .unwrap();
    /// ```
    pub fn show_popup<F>(&self, params: &JsValue, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |id: JsValue| {
            callback(id.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"showPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, params, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.showScanQrPopup(text, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.show_scan_qr_popup("Scan", |text| {
    ///     let _ = text;
    /// })
    /// .unwrap();
    /// ```
    pub fn show_scan_qr_popup<F>(&self, text: &str, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |value: JsValue| {
            callback(value.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"showScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &text.into(), cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.closeScanQrPopup()`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.close_scan_qr_popup().unwrap();
    /// ```
    pub fn close_scan_qr_popup(&self) -> Result<(), JsValue> {
        Reflect::get(&self.inner, &"closeScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call0(&self.inner)?;
        Ok(())
    }

    /// Call `WebApp.hideKeyboard()`.
    fn bottom_button_object(&self, button: BottomButton) -> Result<Object, JsValue> {
        let name = button.js_name();
        Reflect::get(&self.inner, &name.into())
            .inspect_err(|_| logger::error(&format!("{name} not available")))?
            .dyn_into::<Object>()
            .inspect_err(|_| logger::error(&format!("{name} is not an object")))
    }

    fn bottom_button_method(
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

    /// Call `WebApp.readTextFromClipboard(callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.read_text_from_clipboard(|text| {
    ///     let _ = text;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn read_text_from_clipboard<F>(&self, callback: F) -> Result<(), JsValue>
    where
        F: 'static + Fn(String)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |text: JsValue| {
            callback(text.as_string().unwrap_or_default());
        });
        let f = Reflect::get(&self.inner, &"readTextFromClipboard".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("readTextFromClipboard is not a function"))?;
        func.call1(&self.inner, cb.as_ref().unchecked_ref())?;
        cb.forget();
        Ok(())
    }

    /// Call `WebApp.MainButton.show()`.
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

    /// Call `WebApp.ready()`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn ready(&self) -> Result<(), JsValue> {
        self.call0("ready")
    }

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

    /// Set main button text.
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

    /// Legacy alias for [`Self::show_bottom_button`] with
    /// [`BottomButton::Main`].
    pub fn show_main_button(&self) -> Result<(), JsValue> {
        self.show_bottom_button(BottomButton::Main)
    }

    /// Show the secondary bottom button.
    pub fn show_secondary_button(&self) -> Result<(), JsValue> {
        self.show_bottom_button(BottomButton::Secondary)
    }

    /// Legacy alias for [`Self::hide_bottom_button`] with
    /// [`BottomButton::Main`].
    pub fn hide_main_button(&self) -> Result<(), JsValue> {
        self.hide_bottom_button(BottomButton::Main)
    }

    /// Hide the secondary bottom button.
    pub fn hide_secondary_button(&self) -> Result<(), JsValue> {
        self.hide_bottom_button(BottomButton::Secondary)
    }

    /// Legacy alias for [`Self::set_bottom_button_text`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_text(&self, text: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text(BottomButton::Main, text)
    }

    /// Set text for the secondary bottom button.
    pub fn set_secondary_button_text(&self, text: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text(BottomButton::Secondary, text)
    }

    /// Legacy alias for [`Self::set_bottom_button_color`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_color(BottomButton::Main, color)
    }

    /// Set color for the secondary bottom button.
    pub fn set_secondary_button_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_color(BottomButton::Secondary, color)
    }

    /// Legacy alias for [`Self::set_bottom_button_text_color`] with
    /// [`BottomButton::Main`].
    pub fn set_main_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text_color(BottomButton::Main, color)
    }

    /// Set text color for the secondary bottom button.
    pub fn set_secondary_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        self.set_bottom_button_text_color(BottomButton::Secondary, color)
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

    /// Legacy alias for [`Self::remove_bottom_button_callback`].
    pub fn remove_main_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        self.remove_bottom_button_callback(handle)
    }

    /// Remove callback for the secondary bottom button.
    pub fn remove_secondary_button_callback(
        &self,
        handle: EventHandle<dyn FnMut()>
    ) -> Result<(), JsValue> {
        self.remove_bottom_button_callback(handle)
    }

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

    /// Internal: call `this[field][method]()`
    fn call_nested0(&self, field: &str, method: &str) -> Result<(), JsValue> {
        let obj = Reflect::get(&self.inner, &field.into())?;
        let f = Reflect::get(&obj, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&obj)?;
        Ok(())
    }

    // === Internal generic method helpers ===

    fn call0(&self, method: &str) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call0(&self.inner)?;
        Ok(())
    }

    fn call1(&self, method: &str, arg: &JsValue) -> Result<(), JsValue> {
        let f = Reflect::get(&self.inner, &method.into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("not a function"))?;
        func.call1(&self.inner, arg)?;
        Ok(())
    }

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

    pub fn is_expanded(&self) -> bool {
        Reflect::get(&self.inner, &"isExpanded".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Call `WebApp.expand()` to expand the viewport.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn expand_viewport(&self) -> Result<(), JsValue> {
        self.call0("expand")
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
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc
    };

    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_webapp() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        webapp
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_keyboard_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"hideKeyboard".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.hide_keyboard().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_main_button_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &main_button,
            &"hide".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.hide_bottom_button(BottomButton::Main).unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn hide_secondary_button_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let hide_cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &secondary_button,
            &"hide".into(),
            hide_cb.as_ref().unchecked_ref()
        );
        hide_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.hide_bottom_button(BottomButton::Secondary).unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_button_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_color(BottomButton::Main, "#00ff00")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_secondary_button_color_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &secondary_button,
            &"setColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_color(BottomButton::Secondary, "#00ff00")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_button_text_color_calls_js() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &main_button,
            &"setTextColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_text_color(BottomButton::Main, "#112233")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_secondary_button_text_color_calls_js() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let set_color_cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &secondary_button,
            &"setTextColor".into(),
            set_color_cb.as_ref().unchecked_ref()
        );
        set_color_cb.forget();

        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_button_text_color(BottomButton::Secondary, "#112233")
            .unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_header_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setHeaderColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_header_color("#abcdef").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#abcdef"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_background_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setBackgroundColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_background_color("#123456").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#123456"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_bottom_bar_color_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"setBottomBarColor".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.set_bottom_bar_color("#654321").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#654321"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_dimensions() {
        let webapp = setup_webapp();
        let _ = Reflect::set(&webapp, &"viewportWidth".into(), &JsValue::from_f64(320.0));
        let _ = Reflect::set(
            &webapp,
            &"viewportStableHeight".into(),
            &JsValue::from_f64(480.0)
        );
        let app = TelegramWebApp::instance().unwrap();
        assert_eq!(app.viewport_width(), Some(320.0));
        assert_eq!(app.viewport_stable_height(), Some(480.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn back_button_visibility_and_callback() {
        let webapp = setup_webapp();
        let back_button = Object::new();
        let _ = Reflect::set(&webapp, &"BackButton".into(), &back_button);
        let _ = Reflect::set(&back_button, &"isVisible".into(), &JsValue::TRUE);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&back_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&back_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_back_button_visible());
        let handle = app
            .set_back_button_callback(move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&back_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_back_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&back_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn bottom_button_callback_register_and_remove() {
        let webapp = setup_webapp();
        let main_button = Object::new();
        let _ = Reflect::set(&webapp, &"MainButton".into(), &main_button);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&main_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&main_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .set_bottom_button_callback(BottomButton::Main, move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&main_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_bottom_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&main_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn secondary_button_callback_register_and_remove() {
        let webapp = setup_webapp();
        let secondary_button = Object::new();
        let _ = Reflect::set(&webapp, &"SecondaryButton".into(), &secondary_button);

        let on_click = Function::new_with_args("cb", "this.cb = cb;");
        let off_click = Function::new_with_args("", "delete this.cb;");
        let _ = Reflect::set(&secondary_button, &"onClick".into(), &on_click);
        let _ = Reflect::set(&secondary_button, &"offClick".into(), &off_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .set_bottom_button_callback(BottomButton::Secondary, move || {
                called_clone.set(true);
            })
            .unwrap();

        let stored = Reflect::has(&secondary_button, &"cb".into()).unwrap();
        assert!(stored);

        let cb_fn = Reflect::get(&secondary_button, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb_fn.call0(&JsValue::NULL);
        assert!(called.get());

        app.remove_bottom_button_callback(handle).unwrap();
        let stored_after = Reflect::has(&secondary_button, &"cb".into()).unwrap();
        assert!(!stored_after);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn on_event_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_event("test", |_: JsValue| {}).unwrap();
        assert!(Reflect::has(&webapp, &"test".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"test".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn background_event_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app
            .on_background_event(BackgroundEvent::MainButtonClicked, |_| {})
            .unwrap();
        assert!(Reflect::has(&webapp, &"mainButtonClicked".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"mainButtonClicked".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn background_event_delivers_data() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);

        let app = TelegramWebApp::instance().unwrap();
        let received = Rc::new(RefCell::new(String::new()));
        let received_clone = Rc::clone(&received);
        let _handle = app
            .on_background_event(BackgroundEvent::InvoiceClosed, move |v| {
                *received_clone.borrow_mut() = v.as_string().unwrap_or_default();
            })
            .unwrap();

        let cb = Reflect::get(&webapp, &"invoiceClosed".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        let _ = cb.call1(&JsValue::NULL, &JsValue::from_str("paid"));
        assert_eq!(received.borrow().as_str(), "paid");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn theme_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_theme_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"themeChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"themeChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn safe_area_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_safe_area_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"safeAreaChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"safeAreaChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn content_safe_area_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_content_safe_area_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"contentSafeAreaChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"contentSafeAreaChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn viewport_changed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_viewport_changed(|| {}).unwrap();
        assert!(Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"viewportChanged".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn clipboard_text_received_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_clipboard_text_received(|_| {}).unwrap();
        assert!(Reflect::has(&webapp, &"clipboardTextReceived".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"clipboardTextReceived".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_link_and_telegram_link() {
        let webapp = setup_webapp();
        let open_link = Function::new_with_args("url", "this.open_link = url;");
        let open_tg_link = Function::new_with_args("url", "this.open_tg_link = url;");
        let _ = Reflect::set(&webapp, &"openLink".into(), &open_link);
        let _ = Reflect::set(&webapp, &"openTelegramLink".into(), &open_tg_link);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        app.open_link(url).unwrap();
        app.open_telegram_link(url).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"open_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
        assert_eq!(
            Reflect::get(&webapp, &"open_tg_link".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url)
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn invoice_closed_register_and_remove() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let off_event = Function::new_with_args("name", "delete this[name];");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let _ = Reflect::set(&webapp, &"offEvent".into(), &off_event);

        let app = TelegramWebApp::instance().unwrap();
        let handle = app.on_invoice_closed(|_| {}).unwrap();
        assert!(Reflect::has(&webapp, &"invoiceClosed".into()).unwrap());
        app.off_event(handle).unwrap();
        assert!(!Reflect::has(&webapp, &"invoiceClosed".into()).unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn invoice_closed_invokes_callback() {
        let webapp = setup_webapp();
        let on_event = Function::new_with_args("name, cb", "this.cb = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);
        app.on_invoice_closed(move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        let cb = Reflect::get(&webapp, &"cb".into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap();
        cb.call1(&webapp, &"paid".into()).unwrap();
        assert_eq!(status.borrow().as_str(), "paid");
        cb.call1(&webapp, &"failed".into()).unwrap();
        assert_eq!(status.borrow().as_str(), "failed");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn open_invoice_invokes_callback() {
        let webapp = setup_webapp();
        let open_invoice = Function::new_with_args("url, cb", "cb('paid');");
        let _ = Reflect::set(&webapp, &"openInvoice".into(), &open_invoice);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);

        app.open_invoice("https://invoice", move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        assert_eq!(status.borrow().as_str(), "paid");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn switch_inline_query_calls_js() {
        let webapp = setup_webapp();
        let switch_inline =
            Function::new_with_args("query, types", "this.query = query; this.types = types;");
        let _ = Reflect::set(&webapp, &"switchInlineQuery".into(), &switch_inline);

        let app = TelegramWebApp::instance().unwrap();
        let types = JsValue::from_str("users");
        app.switch_inline_query("search", Some(&types)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"query".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("search"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"types".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("users"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_message_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args("id, cb", "this.shared_id = id; cb(true);");
        let _ = Reflect::set(&webapp, &"shareMessage".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let sent = Rc::new(Cell::new(false));
        let sent_clone = Rc::clone(&sent);

        app.share_message("123", move |s| {
            sent_clone.set(s);
        })
        .unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"shared_id".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("123"),
        );
        assert!(sent.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_to_story_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args(
            "url, params",
            "this.story_url = url; this.story_params = params;"
        );
        let _ = Reflect::set(&webapp, &"shareToStory".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com/media";
        let params = Object::new();
        let _ = Reflect::set(&params, &"text".into(), &"hi".into());
        app.share_to_story(url, Some(&params.into())).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"story_url".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url),
        );
        let stored = Reflect::get(&webapp, &"story_params".into()).unwrap();
        assert_eq!(
            Reflect::get(&stored, &"text".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("hi"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn share_url_calls_js() {
        let webapp = setup_webapp();
        let share = Function::new_with_args(
            "url, text",
            "this.shared_url = url; this.shared_text = text;"
        );
        let _ = Reflect::set(&webapp, &"shareURL".into(), &share);

        let app = TelegramWebApp::instance().unwrap();
        let url = "https://example.com";
        let text = "check";
        app.share_url(url, Some(text)).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"shared_url".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(url),
        );
        assert_eq!(
            Reflect::get(&webapp, &"shared_text".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some(text),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn join_voice_chat_calls_js() {
        let webapp = setup_webapp();
        let join = Function::new_with_args(
            "id, hash",
            "this.voice_chat_id = id; this.voice_chat_hash = hash;"
        );
        let _ = Reflect::set(&webapp, &"joinVoiceChat".into(), &join);

        let app = TelegramWebApp::instance().unwrap();
        app.join_voice_chat("123", Some("hash")).unwrap();

        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_id".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("123"),
        );
        assert_eq!(
            Reflect::get(&webapp, &"voice_chat_hash".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("hash"),
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn add_to_home_screen_calls_js() {
        let webapp = setup_webapp();
        let add = Function::new_with_args("", "this.called = true; return true;");
        let _ = Reflect::set(&webapp, &"addToHomeScreen".into(), &add);

        let app = TelegramWebApp::instance().unwrap();
        let shown = app.add_to_home_screen().unwrap();
        assert!(shown);
        let called = Reflect::get(&webapp, &"called".into())
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(called);
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_fullscreen_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"requestFullscreen".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.request_fullscreen().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn exit_fullscreen_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"exitFullscreen".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.exit_fullscreen().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn check_home_screen_status_invokes_callback() {
        let webapp = setup_webapp();
        let check = Function::new_with_args("cb", "cb('added');");
        let _ = Reflect::set(&webapp, &"checkHomeScreenStatus".into(), &check);

        let app = TelegramWebApp::instance().unwrap();
        let status = Rc::new(RefCell::new(String::new()));
        let status_clone = Rc::clone(&status);

        app.check_home_screen_status(move |s| {
            *status_clone.borrow_mut() = s;
        })
        .unwrap();

        assert_eq!(status.borrow().as_str(), "added");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn lock_orientation_calls_js() {
        let webapp = setup_webapp();
        let received = Rc::new(RefCell::new(None));
        let rc_clone = Rc::clone(&received);

        let cb = Closure::<dyn FnMut(JsValue)>::new(move |v: JsValue| {
            *rc_clone.borrow_mut() = v.as_string();
        });
        let _ = Reflect::set(
            &webapp,
            &"lockOrientation".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.lock_orientation("portrait").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("portrait"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn unlock_orientation_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"unlockOrientation".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.unlock_orientation().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn enable_vertical_swipes_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"enableVerticalSwipes".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.enable_vertical_swipes().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn disable_vertical_swipes_calls_js() {
        let webapp = setup_webapp();
        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let cb = Closure::<dyn FnMut()>::new(move || {
            called_clone.set(true);
        });
        let _ = Reflect::set(
            &webapp,
            &"disableVerticalSwipes".into(),
            cb.as_ref().unchecked_ref()
        );
        cb.forget();

        let app = TelegramWebApp::instance().unwrap();
        app.disable_vertical_swipes().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_write_access_invokes_callback() {
        let webapp = setup_webapp();
        let request = Function::new_with_args("cb", "cb(true);");
        let _ = Reflect::set(&webapp, &"requestWriteAccess".into(), &request);

        let app = TelegramWebApp::instance().unwrap();
        let granted = Rc::new(Cell::new(false));
        let granted_clone = Rc::clone(&granted);

        let res = app.request_write_access(move |g| {
            granted_clone.set(g);
        });
        assert!(res.is_ok());

        assert!(granted.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn download_file_invokes_callback() {
        let webapp = setup_webapp();
        let received_url = Rc::new(RefCell::new(String::new()));
        let received_name = Rc::new(RefCell::new(String::new()));
        let url_clone = Rc::clone(&received_url);
        let name_clone = Rc::clone(&received_name);

        let download = Closure::<dyn FnMut(JsValue, JsValue)>::new(move |params, cb: JsValue| {
            let url = Reflect::get(&params, &"url".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            let name = Reflect::get(&params, &"file_name".into())
                .unwrap()
                .as_string()
                .unwrap_or_default();
            *url_clone.borrow_mut() = url;
            *name_clone.borrow_mut() = name;
            let func = cb.dyn_ref::<Function>().unwrap();
            let _ = func.call1(&JsValue::NULL, &JsValue::from_str("id"));
        });
        let _ = Reflect::set(
            &webapp,
            &"downloadFile".into(),
            download.as_ref().unchecked_ref()
        );
        download.forget();

        let app = TelegramWebApp::instance().unwrap();
        let result = Rc::new(RefCell::new(String::new()));
        let result_clone = Rc::clone(&result);
        let params = DownloadFileParams {
            url:       "https://example.com/data.bin",
            file_name: Some("data.bin"),
            mime_type: None
        };
        app.download_file(params, move |id| {
            *result_clone.borrow_mut() = id;
        })
        .unwrap();

        assert_eq!(
            received_url.borrow().as_str(),
            "https://example.com/data.bin"
        );
        assert_eq!(received_name.borrow().as_str(), "data.bin");
        assert_eq!(result.borrow().as_str(), "id");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_write_access_returns_error_when_missing() {
        let _webapp = setup_webapp();
        let app = TelegramWebApp::instance().unwrap();
        let res = app.request_write_access(|_| {});
        assert!(res.is_err());
    }
    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn request_emoji_status_access_invokes_callback() {
        let webapp = setup_webapp();
        let request = Function::new_with_args("cb", "cb(false);");
        let _ = Reflect::set(&webapp, &"requestEmojiStatusAccess".into(), &request);

        let app = TelegramWebApp::instance().unwrap();
        let granted = Rc::new(Cell::new(true));
        let granted_clone = Rc::clone(&granted);

        app.request_emoji_status_access(move |g| {
            granted_clone.set(g);
        })
        .unwrap();

        assert!(!granted.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_emoji_status_invokes_callback() {
        let webapp = setup_webapp();
        let set_status = Function::new_with_args("status, cb", "this.st = status; cb(true);");
        let _ = Reflect::set(&webapp, &"setEmojiStatus".into(), &set_status);

        let status = Object::new();
        let _ = Reflect::set(
            &status,
            &"custom_emoji_id".into(),
            &JsValue::from_str("321")
        );

        let app = TelegramWebApp::instance().unwrap();
        let success = Rc::new(Cell::new(false));
        let success_clone = Rc::clone(&success);

        app.set_emoji_status(&status.into(), move |s| {
            success_clone.set(s);
        })
        .unwrap();

        assert!(success.get());
        let stored = Reflect::get(&webapp, &"st".into()).unwrap();
        let id = Reflect::get(&stored, &"custom_emoji_id".into())
            .unwrap()
            .as_string();
        assert_eq!(id.as_deref(), Some("321"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_popup_invokes_callback() {
        let webapp = setup_webapp();
        let show_popup = Function::new_with_args("params, cb", "cb('ok');");
        let _ = Reflect::set(&webapp, &"showPopup".into(), &show_popup);

        let app = TelegramWebApp::instance().unwrap();
        let button = Rc::new(RefCell::new(String::new()));
        let button_clone = Rc::clone(&button);

        app.show_popup(&JsValue::NULL, move |id| {
            *button_clone.borrow_mut() = id;
        })
        .unwrap();

        assert_eq!(button.borrow().as_str(), "ok");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn read_text_from_clipboard_invokes_callback() {
        let webapp = setup_webapp();
        let read_clip = Function::new_with_args("cb", "cb('clip');");
        let _ = Reflect::set(&webapp, &"readTextFromClipboard".into(), &read_clip);

        let app = TelegramWebApp::instance().unwrap();
        let text = Rc::new(RefCell::new(String::new()));
        let text_clone = Rc::clone(&text);

        app.read_text_from_clipboard(move |t| {
            *text_clone.borrow_mut() = t;
        })
        .unwrap();

        assert_eq!(text.borrow().as_str(), "clip");
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn scan_qr_popup_invokes_callback_and_close() {
        let webapp = setup_webapp();
        let show_scan = Function::new_with_args("text, cb", "cb('code');");
        let close_scan = Function::new_with_args("", "this.closed = true;");
        let _ = Reflect::set(&webapp, &"showScanQrPopup".into(), &show_scan);
        let _ = Reflect::set(&webapp, &"closeScanQrPopup".into(), &close_scan);

        let app = TelegramWebApp::instance().unwrap();
        let text = Rc::new(RefCell::new(String::new()));
        let text_clone = Rc::clone(&text);

        app.show_scan_qr_popup("scan", move |value| {
            *text_clone.borrow_mut() = value;
        })
        .unwrap();
        assert_eq!(text.borrow().as_str(), "code");

        app.close_scan_qr_popup().unwrap();
        let closed = Reflect::get(&webapp, &"closed".into())
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        assert!(closed);
    }
}
