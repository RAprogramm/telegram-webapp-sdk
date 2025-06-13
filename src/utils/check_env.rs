use js_sys::Reflect;
use web_sys::window;

/// Checks if the code is running inside Telegram Mini App.
pub fn is_telegram_env() -> bool {
    let win = match window() {
        Some(w) => w,
        None => return false
    };

    let telegram = Reflect::get(&win, &"Telegram".into());
    if telegram.is_err() || telegram.as_ref().unwrap().is_undefined() {
        return false;
    }

    let webapp = Reflect::get(telegram.as_ref().unwrap(), &"WebApp".into());
    if webapp.is_err() || webapp.as_ref().unwrap().is_undefined() {
        return false;
    }

    true
}
