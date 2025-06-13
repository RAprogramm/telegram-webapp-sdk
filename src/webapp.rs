use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::window;

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
    pub fn viewport_height(&self) -> Option<f64> {
        Reflect::get(&self.inner, &"viewportHeight".into())
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
}
