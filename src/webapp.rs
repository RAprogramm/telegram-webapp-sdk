use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::window;

use crate::logger;

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

    /// Call `WebApp.sendData(data)`
    pub fn send_data(&self, data: &str) {
        let _ = self.call1("sendData", &data.into());
    }

    /// Call `WebApp.expand()`
    pub fn expand(&self) {
        let _ = self.call0("expand");
    }

    /// Call `WebApp.close()`
    pub fn close(&self) {
        let _ = self.call0("close");
    }

    /// Call `WebApp.showAlert(message)`
    pub fn show_alert(&self, msg: &str) {
        let _ = self.call1("showAlert", &msg.into());
    }

    /// Call `WebApp.showConfirm(message, callback)`
    pub fn show_confirm<F>(&self, msg: &str, on_confirm: F)
    where
        F: 'static + Fn(bool)
    {
        let cb = Closure::<dyn FnMut(bool)>::new(on_confirm);
        let _ = Reflect::get(&self.inner, &"showConfirm".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| {
                f.call2(&self.inner, &msg.into(), cb.as_ref().unchecked_ref())
                    .ok()
            });
        cb.forget(); // safe leak for JS lifetime
    }

    /// Call `WebApp.MainButton.show()`
    pub fn show_main_button(&self) {
        if let Ok(main_button) = Reflect::get(&self.inner, &"MainButton".into()) {
            let _ = Reflect::get(&main_button, &"show".into())
                .ok()
                .and_then(|f| f.dyn_ref::<Function>().cloned())
                .and_then(|f| f.call0(&main_button).ok());
        }
    }

    /// Call `WebApp.MainButton.hide()`.
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.hide_main_button();
    /// ```
    pub fn hide_main_button(&self) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let hide = Reflect::get(&main_button, &"hide".into())
            .inspect_err(|_| logger::error("MainButton.hide not available"))?;
        let func = hide
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.hide is not a function"))?;
        func.call0(&main_button)
            .inspect_err(|_| logger::error("MainButton.hide call failed"))?;
        Ok(())
    }
    /// Call `WebApp.ready()`
    pub fn ready(&self) {
        let _ = self.call0("ready");
    }

    /// Show back button
    pub fn show_back_button(&self) {
        self.call_nested0("BackButton", "show");
    }

    /// Hide back button
    pub fn hide_back_button(&self) {
        self.call_nested0("BackButton", "hide");
    }

    /// Set main button text
    pub fn set_main_button_text(&self, text: &str) {
        if let Ok(main_button) = Reflect::get(&self.inner, &"MainButton".into()) {
            let _ = Reflect::get(&main_button, &"setText".into())
                .ok()
                .and_then(|f| f.dyn_ref::<Function>().cloned())
                .and_then(|f| f.call1(&main_button, &text.into()).ok());
        }
    }

    /// Set main button color (`MainButton.setColor(color)`).
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_main_button_color("#ff0000");
    /// ```
    pub fn set_main_button_color(&self, color: &str) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let set_color = Reflect::get(&main_button, &"setColor".into())
            .inspect_err(|_| logger::error("MainButton.setColor not available"))?;
        let func = set_color
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.setColor is not a function"))?;
        func.call1(&main_button, &color.into())
            .inspect_err(|_| logger::error("MainButton.setColor call failed"))?;
        Ok(())
    }

    /// Set main button text color (`MainButton.setTextColor(color)`).
    ///
    /// # Errors
    /// Returns `Err` if the underlying JavaScript call fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// let _ = app.set_main_button_text_color("#ffffff");
    /// ```
    pub fn set_main_button_text_color(&self, color: &str) -> Result<(), JsValue> {
        let main_button = Reflect::get(&self.inner, &"MainButton".into())
            .inspect_err(|_| logger::error("MainButton not available"))?;
        let set_color = Reflect::get(&main_button, &"setTextColor".into())
            .inspect_err(|_| logger::error("MainButton.setTextColor not available"))?;
        let func = set_color
            .dyn_into::<Function>()
            .inspect_err(|_| logger::error("MainButton.setTextColor is not a function"))?;
        func.call1(&main_button, &color.into())
            .inspect_err(|_| logger::error("MainButton.setTextColor call failed"))?;
        Ok(())
    }

    /// Set callback for MainButton.onClick()
    pub fn set_main_button_callback<F>(&self, callback: F)
    where
        F: 'static + Fn()
    {
        if let Ok(main_button) = Reflect::get(&self.inner, &"MainButton".into()) {
            let cb = Closure::<dyn FnMut()>::new(callback);
            let _ = Reflect::get(&main_button, &"onClick".into())
                .ok()
                .and_then(|f| f.dyn_ref::<Function>().cloned())
                .and_then(|f| f.call1(&main_button, cb.as_ref().unchecked_ref()).ok());
            cb.forget(); // Safe leak
        }
    }

    /// Register event handler (web_app_event_name, callback)
    pub fn on_event<F>(&self, event: &str, callback: F)
    where
        F: 'static + Fn(JsValue)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(callback);
        let _ = Reflect::get(&self.inner, &"onEvent".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| {
                f.call2(&self.inner, &event.into(), cb.as_ref().unchecked_ref())
                    .ok()
            });
        cb.forget(); // Safe leak
    }

    /// Deregister event handler
    pub fn off_event<F>(&self, event: &str, callback: F)
    where
        F: 'static + Fn(JsValue)
    {
        let cb = Closure::<dyn FnMut(JsValue)>::new(callback);
        let _ = Reflect::get(&self.inner, &"offEvent".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| {
                f.call2(&self.inner, &event.into(), cb.as_ref().unchecked_ref())
                    .ok()
            });
    }

    /// Internal: call `this[field][method]()`
    fn call_nested0(&self, field: &str, method: &str) {
        if let Ok(obj) = Reflect::get(&self.inner, &field.into()) {
            let _ = Reflect::get(&obj, &method.into())
                .ok()
                .and_then(|f| f.dyn_ref::<Function>().cloned())
                .and_then(|f| f.call0(&obj).ok());
        }
    }

    // === Internal generic method helpers ===

    fn call0(&self, method: &str) -> Option<()> {
        let f = Reflect::get(&self.inner, &method.into()).ok()?;
        let func = f.dyn_ref::<Function>()?;
        func.call0(&self.inner).ok()?;
        Some(())
    }

    fn call1(&self, method: &str, arg: &JsValue) -> Option<()> {
        let f = Reflect::get(&self.inner, &method.into()).ok()?;
        let func = f.dyn_ref::<Function>()?;
        func.call1(&self.inner, arg).ok()?;
        Some(())
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

    pub fn expand_viewport(&self) {
        let _ = self.call0("expand");
    }

    pub fn on_viewport_changed<F>(&self, callback: F)
    where
        F: 'static + Fn()
    {
        let cb = Closure::<dyn FnMut()>::new(callback);
        let _ = Reflect::get(&self.inner, &"onEvent".into())
            .ok()
            .and_then(|f| f.dyn_ref::<Function>().cloned())
            .and_then(|f| {
                f.call2(
                    &self.inner,
                    &"viewportChanged".into(),
                    cb.as_ref().unchecked_ref()
                )
                .ok()
            });
        cb.forget();
    }

    /// Registers a callback for the native back button.
    ///
    /// # Examples
    /// ```no_run
    /// # use telegram_webapp_sdk::webapp::TelegramWebApp;
    /// # let app = TelegramWebApp::instance().unwrap();
    /// app.set_back_button_callback(|| {});
    /// ```
    pub fn set_back_button_callback<F>(&self, callback: F)
    where
        F: 'static + Fn()
    {
        if let Ok(back_button) = Reflect::get(&self.inner, &"BackButton".into()) {
            let cb = Closure::<dyn FnMut()>::new(callback);
            let _ = Reflect::get(&back_button, &"onClick".into())
                .ok()
                .and_then(|f| f.dyn_ref::<Function>().cloned())
                .and_then(|f| f.call1(&back_button, cb.as_ref().unchecked_ref()).ok());
            cb.forget();
        }
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
        app.hide_main_button().unwrap();
        assert!(called.get());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_main_button_color_calls_js() {
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
        app.set_main_button_color("#00ff00").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#00ff00"));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code, clippy::unused_unit)]
    fn set_main_button_text_color_calls_js() {
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
        app.set_main_button_text_color("#112233").unwrap();
        assert_eq!(received.borrow().as_deref(), Some("#112233"));
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

        let on_click = Function::new_with_args("cb", "cb();");
        let _ = Reflect::set(&back_button, &"onClick".into(), &on_click);

        let called = Rc::new(Cell::new(false));
        let called_clone = Rc::clone(&called);

        let app = TelegramWebApp::instance().unwrap();
        assert!(app.is_back_button_visible());
        app.set_back_button_callback(move || {
            called_clone.set(true);
        });
        assert!(called.get());
    }
}
