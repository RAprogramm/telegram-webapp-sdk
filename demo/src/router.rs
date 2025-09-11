use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::{Event, EventTarget, window};

type RenderFn = fn();

/// Struct managing routing table
pub struct Router {
    routes: HashMap<String, RenderFn>
}

impl Router {
    /// Creates a new router instance
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    /// Registers a path and its render function
    pub fn register(mut self, path: &str, render: RenderFn) -> Self {
        self.routes.insert(path.to_string(), render);
        self
    }

    /// Starts the router: renders initial route and listens to popstate
    pub fn start(&self) {
        self.render_current();

        if let Some(w) = window() {
            let closure = Closure::<dyn FnMut(_)>::new({
                let router = self.routes.clone();
                move |_event: Event| {
                    if let Some(path) = current_path() {
                        if let Some(page) = router.get(&path) {
                            page();
                        }
                    }
                }
            });

            let target: EventTarget = w.into();
            target
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget(); // Safe leak
        }
    }

    /// Renders current route based on location.pathname
    fn render_current(&self) {
        if let Some(path) = current_path() {
            if let Some(page) = self.routes.get(&path) {
                page();
            } else {
                log(&format!("⚠️ Route not found: {}", path));
            }
        }
    }
}

/// Navigates to given path and pushes state
pub fn navigate(path: &str) {
    if let Some(w) = window() {
        if let Ok(history) = w.history() {
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(path));
            // Trigger route manually
            if let Some(event) = web_sys::CustomEvent::new("popstate").ok() {
                let _ = w.dispatch_event(&event);
            }
        }
    }
}

/// Returns current pathname (e.g., "/init-data")
fn current_path() -> Option<String> {
    window()?.location().pathname().ok()
}

/// Logs to console
fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}
