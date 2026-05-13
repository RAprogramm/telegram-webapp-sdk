// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::window;

use crate::{core::context::TelegramContext, webapp::TelegramWebApp};

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

    /// Call `WebApp.invokeCustomMethod(method, params, callback)`.
    ///
    /// The JS callback is `(error, result)`; the wrapper translates it into
    /// `Result<JsValue, JsValue>` for the Rust closure.
    ///
    /// # Examples
    /// ```no_run
    /// # use js_sys::Object;
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let params = Object::new();
    /// app.invoke_custom_method("getRequestedContact", &params.into(), |outcome| {
    ///     let _ = outcome;
    /// })
    /// .unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns [`JsValue`] if the underlying JS call fails.
    pub fn invoke_custom_method<F>(
        &self,
        method: &str,
        params: &JsValue,
        callback: F
    ) -> Result<(), JsValue>
    where
        F: 'static + FnOnce(Result<JsValue, JsValue>)
    {
        let cb = Closure::once_into_js(move |err: JsValue, result: JsValue| {
            if err.is_null() || err.is_undefined() {
                callback(Ok(result));
            } else {
                callback(Err(err));
            }
        });
        let f = Reflect::get(&self.inner, &"invokeCustomMethod".into())?;
        let func = f
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str("invokeCustomMethod is not a function"))?;
        func.call3(&self.inner, &method.into(), params, &cb)?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

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
    fn invoke_custom_method_passes_args_and_delivers_result() {
        let webapp = setup_webapp();
        let invoke = Function::new_with_args(
            "method, params, cb",
            "this.method = method; this.params = params; cb(null, {ok: 1});"
        );
        let _ = Reflect::set(&webapp, &"invokeCustomMethod".into(), &invoke);

        let app = TelegramWebApp::instance().expect("instance");
        let received = Rc::new(RefCell::new(None::<JsValue>));
        let cap = received.clone();
        let params = Object::new();
        let _ = Reflect::set(&params, &"x".into(), &"y".into());
        app.invoke_custom_method("doStuff", &params.into(), move |out| {
            *cap.borrow_mut() = Some(out.expect("ok"));
        })
        .expect("ok");

        assert_eq!(
            Reflect::get(&webapp, &"method".into())
                .unwrap()
                .as_string()
                .as_deref(),
            Some("doStuff")
        );
        let value = received.borrow().clone().expect("result");
        let ok_val = Reflect::get(&value, &"ok".into()).expect("ok field");
        assert_eq!(ok_val.as_f64(), Some(1.0));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn invoke_custom_method_translates_error() {
        let webapp = setup_webapp();
        let invoke = Function::new_with_args("_method, _params, cb", "cb('boom', null);");
        let _ = Reflect::set(&webapp, &"invokeCustomMethod".into(), &invoke);

        let app = TelegramWebApp::instance().expect("instance");
        let received = Rc::new(RefCell::new(None::<JsValue>));
        let cap = received.clone();
        app.invoke_custom_method("doStuff", &JsValue::NULL, move |out| {
            *cap.borrow_mut() = Some(out.expect_err("err"));
        })
        .expect("ok");

        let err = received.borrow().clone().expect("err");
        assert_eq!(err.as_string().as_deref(), Some("boom"));
    }
}
