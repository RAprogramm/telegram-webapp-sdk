// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::webapp::{
    TelegramWebApp,
    core::{await_one_shot, one_shot_promise}
};

impl TelegramWebApp {
    /// Call `WebApp.showAlert(message)`.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_alert(&self, msg: &str) -> Result<(), JsValue> {
        self.call1("showAlert", &msg.into())
    }

    /// Callback variant of [`Self::show_confirm`].
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn show_confirm_with_callback<F>(&self, msg: &str, on_confirm: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(bool)
    {
        let cb = Closure::once_into_js(move |v: JsValue| {
            on_confirm(v.as_bool().unwrap_or(false));
        });
        let f = Reflect::get(&self.inner, &"showConfirm".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("showConfirm is not a function"))?;
        func.call2(&self.inner, &msg.into(), &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.showConfirm`. Resolves with the user's
    /// boolean answer.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn show_confirm(&self, msg: &str) -> Result<bool, JsValue> {
        let webapp = self.inner.clone();
        let msg = msg.to_owned();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |v: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &v);
            });
            let f = Reflect::get(&webapp, &"showConfirm".into())?;
            let func = f
                .dyn_ref::<Function>()
                .ok_or_else(|| JsValue::from_str("showConfirm is not a function"))?;
            func.call2(&webapp, &msg.into(), &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_bool().unwrap_or(false))
    }

    /// Call `WebApp.showPopup(params, callback)`.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.show_popup_with_callback(&params.into(), |id| {
    ///     let _ = id;
    /// })
    /// .unwrap();
    /// ```
    /// Callback variant of [`Self::show_popup`].
    pub fn show_popup_with_callback<F>(&self, params: &JsValue, callback: F) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let cb = Closure::once_into_js(move |id: JsValue| {
            callback(id.as_string().unwrap_or_default());
        });
        Reflect::get(&self.inner, &"showPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, params, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.showPopup`. Resolves with the id of the
    /// button the user pressed, or an empty string if the popup was dismissed.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn show_popup(&self, params: &JsValue) -> Result<String, JsValue> {
        let webapp = self.inner.clone();
        let params = params.clone();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |id: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &id);
            });
            Reflect::get(&webapp, &"showPopup".into())?
                .dyn_into::<Function>()?
                .call2(&webapp, &params, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
    }

    /// Call `WebApp.showScanQrPopup({ text }, callback)`.
    ///
    /// The text is shown above the scanner viewport. Pass an empty string to
    /// open the scanner without a caption.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.show_scan_qr_popup_with_callback("Scan", |text| {
    ///     let _ = text;
    /// })
    /// .unwrap();
    /// ```
    /// Callback variant of [`Self::show_scan_qr_popup`].
    pub fn show_scan_qr_popup_with_callback<F>(
        &self,
        text: &str,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(String)
    {
        let cb = Closure::once_into_js(move |value: JsValue| {
            callback(value.as_string().unwrap_or_default());
        });
        let params = Object::new();
        Reflect::set(&params, &"text".into(), &text.into())?;
        Reflect::get(&self.inner, &"showScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &params, &cb)?;
        Ok(())
    }

    /// Async wrapper over `WebApp.showScanQrPopup`. Resolves with the scanned
    /// text. Pass an empty `text` to open the scanner without a caption.
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub async fn show_scan_qr_popup(&self, text: &str) -> Result<String, JsValue> {
        let webapp = self.inner.clone();
        let text = text.to_owned();
        let promise = one_shot_promise(move |resolve, _reject| {
            let cb = Closure::once_into_js(move |value: JsValue| {
                let _ = resolve.call1(&JsValue::NULL, &value);
            });
            let params = Object::new();
            Reflect::set(&params, &"text".into(), &text.into())?;
            Reflect::get(&webapp, &"showScanQrPopup".into())?
                .dyn_into::<Function>()?
                .call2(&webapp, &params, &cb)?;
            Ok(())
        });
        let value = await_one_shot(promise).await?;
        Ok(value.as_string().unwrap_or_default())
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
}

#[cfg(test)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::JsValue;
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
    fn show_scan_qr_popup_passes_params_as_object_with_text() {
        let webapp = setup_webapp();
        let capture = Function::new_with_args("params, _cb", "this.captured_params = params;");
        let _ = Reflect::set(&webapp, &"showScanQrPopup".into(), &capture);

        let app = TelegramWebApp::instance().expect("instance");
        app.show_scan_qr_popup_with_callback("Scan", |_| {})
            .expect("ok");

        let params = Reflect::get(&webapp, &"captured_params".into()).expect("captured");
        assert!(!params.is_undefined(), "scan params must be an object");
        let text = Reflect::get(&params, &"text".into())
            .expect("text field")
            .as_string();
        assert_eq!(text.as_deref(), Some("Scan"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_alert_passes_message() {
        let webapp = setup_webapp();
        let capture = Function::new_with_args("msg", "this.captured_alert = msg;");
        let _ = Reflect::set(&webapp, &"showAlert".into(), &capture);

        let app = TelegramWebApp::instance().expect("instance");
        app.show_alert("Heads up").expect("ok");

        assert_eq!(
            Reflect::get(&webapp, &"captured_alert".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("Heads up")
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_confirm_passes_message_and_routes_boolean_back() {
        let webapp = setup_webapp();
        let invoke = Function::new_with_args("msg, cb", "this.captured_confirm = msg; cb(true);");
        let _ = Reflect::set(&webapp, &"showConfirm".into(), &invoke);

        let app = TelegramWebApp::instance().expect("instance");
        let received = std::rc::Rc::new(std::cell::Cell::new(false));
        let received_ref = received.clone();
        app.show_confirm_with_callback("Proceed?", move |ok| received_ref.set(ok))
            .expect("ok");

        assert_eq!(
            Reflect::get(&webapp, &"captured_confirm".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("Proceed?")
        );
        assert!(received.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn close_scan_qr_popup_calls_js() {
        let webapp = setup_webapp();
        let called = std::rc::Rc::new(std::cell::Cell::new(false));
        let called_ref = called.clone();
        let close =
            wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || called_ref.set(true));
        let _ = Reflect::set(
            &webapp,
            &"closeScanQrPopup".into(),
            wasm_bindgen::JsCast::unchecked_ref::<Function>(close.as_ref())
        );
        close.forget();

        let app = TelegramWebApp::instance().expect("instance");
        app.close_scan_qr_popup().expect("ok");
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn show_scan_qr_popup_callback_receives_scanned_text() {
        let webapp = setup_webapp();
        // Synchronously invoke the callback with a scanned value so we can
        // observe it without scheduling.
        let invoke = Function::new_with_args("_params, cb", "cb('payload');");
        let _ = Reflect::set(&webapp, &"showScanQrPopup".into(), &invoke);

        let app = TelegramWebApp::instance().expect("instance");
        let captured = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let captured_ref = captured.clone();
        app.show_scan_qr_popup_with_callback("", move |t| {
            *captured_ref.borrow_mut() = t;
        })
        .expect("ok");

        assert_eq!(captured.borrow().as_str(), "payload");
        let _ = JsValue::null();
    }
}
