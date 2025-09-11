use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

/// Available styles for [`impact_occurred`].
#[derive(Debug, Clone, Copy)]
pub enum HapticImpactStyle {
    /// A light impact feedback.
    Light,
    /// A medium impact feedback.
    Medium,
    /// A heavy impact feedback.
    Heavy,
    /// A rigid impact feedback.
    Rigid,
    /// A soft impact feedback.
    Soft
}

impl HapticImpactStyle {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Heavy => "heavy",
            Self::Rigid => "rigid",
            Self::Soft => "soft"
        }
    }
}

/// Available types for [`notification_occurred`].
#[derive(Debug, Clone, Copy)]
pub enum HapticNotificationType {
    /// Error notification feedback.
    Error,
    /// Success notification feedback.
    Success,
    /// Warning notification feedback.
    Warning
}

impl HapticNotificationType {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Success => "success",
            Self::Warning => "warning"
        }
    }
}

/// Triggers a haptic impact feedback.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `HapticFeedback` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::haptic::{HapticImpactStyle, impact_occurred};
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// impact_occurred(HapticImpactStyle::Light)?;
/// # Ok(()) }
/// ```
pub fn impact_occurred(style: HapticImpactStyle) -> Result<(), JsValue> {
    let haptic = haptic_object()?;
    let func = Reflect::get(&haptic, &"impactOccurred".into())?.dyn_into::<Function>()?;
    func.call1(&haptic, &JsValue::from_str(style.as_str()))?;
    Ok(())
}

/// Triggers a haptic notification feedback.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `HapticFeedback` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::haptic::{HapticNotificationType, notification_occurred};
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// notification_occurred(HapticNotificationType::Success)?;
/// # Ok(()) }
/// ```
pub fn notification_occurred(ty: HapticNotificationType) -> Result<(), JsValue> {
    let haptic = haptic_object()?;
    let func = Reflect::get(&haptic, &"notificationOccurred".into())?.dyn_into::<Function>()?;
    func.call1(&haptic, &JsValue::from_str(ty.as_str()))?;
    Ok(())
}

/// Triggers a haptic selection change feedback.
///
/// # Errors
/// Returns `Err(JsValue)` if the JavaScript call fails or `HapticFeedback` is
/// missing.
///
/// # Examples
/// ```
/// use telegram_webapp_sdk::api::haptic::selection_changed;
/// # fn run() -> Result<(), wasm_bindgen::JsValue> {
/// selection_changed()?;
/// # Ok(()) }
/// ```
pub fn selection_changed() -> Result<(), JsValue> {
    let haptic = haptic_object()?;
    let func = Reflect::get(&haptic, &"selectionChanged".into())?.dyn_into::<Function>()?;
    func.call0(&haptic)?;
    Ok(())
}

/// Internal helper to get `Telegram.WebApp.HapticFeedback` object.
fn haptic_object() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&window, &"Telegram".into())?;
    let webapp = Reflect::get(&tg, &"WebApp".into())?;
    Reflect::get(&webapp, &"HapticFeedback".into())
}

#[cfg(test)]
mod tests {
    use js_sys::{Object, Reflect};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_haptic() -> Object {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let haptic = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"HapticFeedback".into(), &haptic);
        haptic
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn impact_calls_js() {
        let haptic = setup_haptic();
        let _ = Reflect::set(&haptic, &"impact_called".into(), &JsValue::FALSE);
        let haptic_clone = haptic.clone();
        let closure = Closure::wrap(Box::new(move |_style: JsValue| {
            let _ = Reflect::set(&haptic_clone, &"impact_called".into(), &JsValue::TRUE);
        }) as Box<dyn FnMut(JsValue)>);
        let _ = Reflect::set(&haptic, &"impactOccurred".into(), closure.as_ref());
        closure.forget();
        let _ = impact_occurred(HapticImpactStyle::Light);
        let flag = Reflect::get(&haptic, &"impact_called".into()).unwrap();
        assert!(flag.as_bool().unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn notification_calls_js() {
        let haptic = setup_haptic();
        let _ = Reflect::set(&haptic, &"notification_called".into(), &JsValue::FALSE);
        let haptic_clone = haptic.clone();
        let closure = Closure::wrap(Box::new(move |_ty: JsValue| {
            let _ = Reflect::set(&haptic_clone, &"notification_called".into(), &JsValue::TRUE);
        }) as Box<dyn FnMut(JsValue)>);
        let _ = Reflect::set(&haptic, &"notificationOccurred".into(), closure.as_ref());
        closure.forget();
        let _ = notification_occurred(HapticNotificationType::Error);
        let flag = Reflect::get(&haptic, &"notification_called".into()).unwrap();
        assert!(flag.as_bool().unwrap());
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    fn selection_calls_js() {
        let haptic = setup_haptic();
        let _ = Reflect::set(&haptic, &"selection_called".into(), &JsValue::FALSE);
        let haptic_clone = haptic.clone();
        let closure = Closure::wrap(Box::new(move || {
            let _ = Reflect::set(&haptic_clone, &"selection_called".into(), &JsValue::TRUE);
        }) as Box<dyn FnMut()>);
        let _ = Reflect::set(&haptic, &"selectionChanged".into(), closure.as_ref());
        closure.forget();
        let _ = selection_changed();
        let flag = Reflect::get(&haptic, &"selection_called".into()).unwrap();
        assert!(flag.as_bool().unwrap());
    }
}
