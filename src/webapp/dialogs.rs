// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::webapp::TelegramWebApp;

impl TelegramWebApp {
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

    /// Call `WebApp.showScanQrPopup({ text }, callback)`.
    ///
    /// The text is shown above the scanner viewport. Pass an empty string to
    /// open the scanner without a caption.
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
        let params = Object::new();
        Reflect::set(&params, &"text".into(), &text.into())?;
        Reflect::get(&self.inner, &"showScanQrPopup".into())?
            .dyn_into::<Function>()?
            .call2(&self.inner, &params, cb.as_ref().unchecked_ref())?;
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
        app.show_scan_qr_popup("Scan", |_| {}).expect("ok");

        let params = Reflect::get(&webapp, &"captured_params".into()).expect("captured");
        assert!(!params.is_undefined(), "scan params must be an object");
        let text = Reflect::get(&params, &"text".into())
            .expect("text field")
            .as_string();
        assert_eq!(text.as_deref(), Some("Scan"));
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
        app.show_scan_qr_popup("", move |t| {
            *captured_ref.borrow_mut() = t;
        })
        .expect("ok");

        assert_eq!(captured.borrow().as_str(), "payload");
        let _ = JsValue::null();
    }
}
