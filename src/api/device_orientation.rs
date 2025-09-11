use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

use super::events;

/// Device orientation angles in degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orientation {
    /// Rotation around the Z axis.
    pub alpha: f64,
    /// Rotation around the X axis.
    pub beta:  f64,
    /// Rotation around the Y axis.
    pub gamma: f64
}

impl Orientation {
    #[allow(dead_code)]
    const fn new(alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            alpha,
            beta,
            gamma
        }
    }
}

/// Starts the device orientation sensor.
///
/// # Errors
/// Returns [`JsValue`] if the JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::device_orientation::start;
/// start()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn start() -> Result<(), JsValue> {
    let orientation = device_orientation_object()?;
    let func = Reflect::get(&orientation, &"start".into())?.dyn_into::<Function>()?;
    func.call0(&orientation)?;
    Ok(())
}

/// Stops the device orientation sensor.
///
/// # Errors
/// Returns [`JsValue`] if the JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::device_orientation::stop;
/// stop()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn stop() -> Result<(), JsValue> {
    let orientation = device_orientation_object()?;
    let func = Reflect::get(&orientation, &"stop".into())?.dyn_into::<Function>()?;
    func.call0(&orientation)?;
    Ok(())
}

/// Reads the current device orientation angles.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::device_orientation::get_orientation;
/// let _ = get_orientation();
/// ```
pub fn get_orientation() -> Option<Orientation> {
    let orientation = device_orientation_object().ok()?;
    let alpha = Reflect::get(&orientation, &"alpha".into()).ok()?.as_f64()?;
    let beta = Reflect::get(&orientation, &"beta".into()).ok()?.as_f64()?;
    let gamma = Reflect::get(&orientation, &"gamma".into()).ok()?.as_f64()?;
    Some(Orientation {
        alpha,
        beta,
        gamma
    })
}

/// Registers a callback for `deviceOrientationStarted` event.
pub fn on_started(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("deviceOrientationStarted", callback)
}

/// Registers a callback for `deviceOrientationChanged` event.
pub fn on_changed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("deviceOrientationChanged", callback)
}

/// Registers a callback for `deviceOrientationStopped` event.
pub fn on_stopped(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("deviceOrientationStopped", callback)
}

/// Registers a callback for `deviceOrientationFailed` event.
pub fn on_failed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("deviceOrientationFailed", callback)
}

fn device_orientation_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &"Telegram".into())?;
    let webapp = Reflect::get(&tg, &"WebApp".into())?;
    Reflect::get(&webapp, &"DeviceOrientation".into())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use js_sys::{Function, Object, Reflect};
    use wasm_bindgen::{JsValue, closure::Closure};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use web_sys::window;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    fn setup_device_orientation() -> (Object, Object) {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let orientation = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"DeviceOrientation".into(), &orientation);
        (webapp, orientation)
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_ok() {
        let (_webapp, orientation) = setup_device_orientation();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&orientation, &"start".into(), &func);
        assert!(start().is_ok());
        let called = Reflect::get(&orientation, &"called".into()).unwrap();
        assert_eq!(called.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_err() {
        let (_webapp, orientation) = setup_device_orientation();
        let _ = Reflect::set(&orientation, &"start".into(), &JsValue::from_f64(1.0));
        assert!(start().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn stop_ok() {
        let (_webapp, orientation) = setup_device_orientation();
        let func = Function::new_no_args("this.stopped = true;");
        let _ = Reflect::set(&orientation, &"stop".into(), &func);
        assert!(stop().is_ok());
        let stopped = Reflect::get(&orientation, &"stopped".into()).unwrap();
        assert_eq!(stopped.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    fn get_orientation_ok() {
        let (_webapp, orientation) = setup_device_orientation();
        let _ = Reflect::set(&orientation, &"alpha".into(), &JsValue::from_f64(10.0));
        let _ = Reflect::set(&orientation, &"beta".into(), &JsValue::from_f64(20.0));
        let _ = Reflect::set(&orientation, &"gamma".into(), &JsValue::from_f64(30.0));
        let result = get_orientation().unwrap();
        assert_eq!(
            result,
            Orientation {
                alpha: 10.0,
                beta:  20.0,
                gamma: 30.0
            }
        );
    }

    #[wasm_bindgen_test]
    fn registers_callbacks() {
        let (webapp, _orientation) = setup_device_orientation();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_started(&cb).expect("on_started");
        on_changed(&cb).expect("on_changed");
        on_stopped(&cb).expect("on_stopped");
        on_failed(&cb).expect("on_failed");
        assert!(Reflect::has(&webapp, &"deviceOrientationStarted".into()).unwrap());
        assert!(Reflect::has(&webapp, &"deviceOrientationChanged".into()).unwrap());
        assert!(Reflect::has(&webapp, &"deviceOrientationStopped".into()).unwrap());
        assert!(Reflect::has(&webapp, &"deviceOrientationFailed".into()).unwrap());
        cb.forget();
    }
}
