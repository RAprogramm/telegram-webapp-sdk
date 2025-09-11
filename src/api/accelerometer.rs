use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

use super::events;

/// Three-dimensional acceleration in meters per second squared.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration {
    /// Acceleration along the X axis.
    pub x: f64,
    /// Acceleration along the Y axis.
    pub y: f64,
    /// Acceleration along the Z axis.
    pub z: f64
}

impl Acceleration {
    /// Creates a new [`Acceleration`] instance.
    #[allow(dead_code)]
    const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

/// Starts the accelerometer.
///
/// # Errors
/// Returns [`JsValue`] if the underlying JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::accelerometer::start;
/// start()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn start() -> Result<(), JsValue> {
    let accel = accelerometer_object()?;
    let func = Reflect::get(&accel, &"start".into())?.dyn_into::<Function>()?;
    func.call0(&accel)?;
    Ok(())
}

/// Stops the accelerometer.
///
/// # Errors
/// Returns [`JsValue`] if the underlying JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::accelerometer::stop;
/// stop()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn stop() -> Result<(), JsValue> {
    let accel = accelerometer_object()?;
    let func = Reflect::get(&accel, &"stop".into())?.dyn_into::<Function>()?;
    func.call0(&accel)?;
    Ok(())
}

/// Reads the current acceleration values.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::accelerometer::get_acceleration;
/// let _ = get_acceleration();
/// ```
pub fn get_acceleration() -> Option<Acceleration> {
    let accel = accelerometer_object().ok()?;
    let x = Reflect::get(&accel, &"x".into()).ok()?.as_f64()?;
    let y = Reflect::get(&accel, &"y".into()).ok()?.as_f64()?;
    let z = Reflect::get(&accel, &"z".into()).ok()?.as_f64()?;
    Some(Acceleration {
        x,
        y,
        z
    })
}

/// Registers a callback for `accelerometerStarted` event.
///
/// ⚠️ The closure must be kept alive for as long as it is needed.
pub fn on_started(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("accelerometerStarted", callback)
}

/// Registers a callback for `accelerometerChanged` event.
///
/// ⚠️ The closure must be kept alive for as long as it is needed.
pub fn on_changed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("accelerometerChanged", callback)
}

/// Registers a callback for `accelerometerStopped` event.
///
/// ⚠️ The closure must be kept alive for as long as it is needed.
pub fn on_stopped(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("accelerometerStopped", callback)
}

/// Registers a callback for `accelerometerFailed` event.
///
/// ⚠️ The closure must be kept alive for as long as it is needed.
pub fn on_failed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("accelerometerFailed", callback)
}

fn accelerometer_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &"Telegram".into())?;
    let webapp = Reflect::get(&tg, &"WebApp".into())?;
    Reflect::get(&webapp, &"Accelerometer".into())
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
    fn setup_accelerometer() -> (Object, Object) {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let accel = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"Accelerometer".into(), &accel);
        (webapp, accel)
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_ok() {
        let (_webapp, accel) = setup_accelerometer();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&accel, &"start".into(), &func);
        assert!(start().is_ok());
        let called = Reflect::get(&accel, &"called".into()).unwrap();
        assert_eq!(called.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_err() {
        let (_webapp, accel) = setup_accelerometer();
        let _ = Reflect::set(&accel, &"start".into(), &JsValue::from_f64(1.0));
        assert!(start().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn stop_ok() {
        let (_webapp, accel) = setup_accelerometer();
        let func = Function::new_no_args("this.stopped = true;");
        let _ = Reflect::set(&accel, &"stop".into(), &func);
        assert!(stop().is_ok());
        let stopped = Reflect::get(&accel, &"stopped".into()).unwrap();
        assert_eq!(stopped.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    fn get_acceleration_ok() {
        let (_webapp, accel) = setup_accelerometer();
        let _ = Reflect::set(&accel, &"x".into(), &JsValue::from_f64(1.0));
        let _ = Reflect::set(&accel, &"y".into(), &JsValue::from_f64(2.0));
        let _ = Reflect::set(&accel, &"z".into(), &JsValue::from_f64(3.0));
        let result = get_acceleration().unwrap();
        assert_eq!(
            result,
            Acceleration {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[wasm_bindgen_test]
    fn registers_callbacks() {
        let (webapp, _accel) = setup_accelerometer();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_started(&cb).expect("on_started");
        on_changed(&cb).expect("on_changed");
        on_stopped(&cb).expect("on_stopped");
        on_failed(&cb).expect("on_failed");
        assert!(Reflect::has(&webapp, &"accelerometerStarted".into()).unwrap());
        assert!(Reflect::has(&webapp, &"accelerometerChanged".into()).unwrap());
        assert!(Reflect::has(&webapp, &"accelerometerStopped".into()).unwrap());
        assert!(Reflect::has(&webapp, &"accelerometerFailed".into()).unwrap());
        cb.forget();
    }
}
