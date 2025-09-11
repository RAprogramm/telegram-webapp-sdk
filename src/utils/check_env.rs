use js_sys::Reflect;
use web_sys::window;

/// Checks if the code is running inside Telegram Mini App.
pub fn is_telegram_env() -> bool {
    let win = match window() {
        Some(w) => w,
        None => return false
    };

    let telegram = match Reflect::get(&win, &"Telegram".into()) {
        Ok(v) if !v.is_undefined() => v,
        _ => return false
    };

    let _webapp = match Reflect::get(&telegram, &"WebApp".into()) {
        Ok(v) if !v.is_undefined() => v,
        _ => return false
    };

    true
}

#[cfg(test)]
mod tests {
    use js_sys::{Object, Reflect};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn cleanup() {
        let win = window().unwrap();
        let _ = Reflect::delete_property(&win, &"Telegram".into());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn returns_false_without_telegram() {
        cleanup();
        assert!(!is_telegram_env());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn returns_false_without_webapp() {
        cleanup();
        let win = window().unwrap();
        let telegram = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        assert!(!is_telegram_env());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn returns_true_with_telegram_and_webapp() {
        cleanup();
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        assert!(is_telegram_env());
    }
}
