use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::window;

use super::events;

/// Angular velocity around three axes in radians per second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularVelocity {
    /// Rotation rate around the X axis.
    pub x: f64,
    /// Rotation rate around the Y axis.
    pub y: f64,
    /// Rotation rate around the Z axis.
    pub z: f64
}

impl AngularVelocity {
    #[allow(dead_code)]
    const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

/// Starts the gyroscope.
///
/// # Errors
/// Returns [`JsValue`] if the JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::gyroscope::start;
/// start()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn start() -> Result<(), JsValue> {
    let gyro = gyroscope_object()?;
    let func = Reflect::get(&gyro, &"start".into())?.dyn_into::<Function>()?;
    func.call0(&gyro)?;
    Ok(())
}

/// Stops the gyroscope.
///
/// # Errors
/// Returns [`JsValue`] if the JavaScript call fails or the sensor is
/// unavailable.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::gyroscope::stop;
/// stop()?;
/// # Ok::<(), wasm_bindgen::JsValue>(())
/// ```
pub fn stop() -> Result<(), JsValue> {
    let gyro = gyroscope_object()?;
    let func = Reflect::get(&gyro, &"stop".into())?.dyn_into::<Function>()?;
    func.call0(&gyro)?;
    Ok(())
}

/// Reads the current angular velocity values.
///
/// # Examples
/// ```no_run
/// # use telegram_webapp_sdk::api::gyroscope::get_angular_velocity;
/// let _ = get_angular_velocity();
/// ```
pub fn get_angular_velocity() -> Option<AngularVelocity> {
    let gyro = gyroscope_object().ok()?;
    let x = Reflect::get(&gyro, &"x".into()).ok()?.as_f64()?;
    let y = Reflect::get(&gyro, &"y".into()).ok()?.as_f64()?;
    let z = Reflect::get(&gyro, &"z".into()).ok()?.as_f64()?;
    Some(AngularVelocity {
        x,
        y,
        z
    })
}

/// Registers a callback for `gyroscopeStarted` event.
pub fn on_started(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("gyroscopeStarted", callback)
}

/// Registers a callback for `gyroscopeChanged` event.
pub fn on_changed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("gyroscopeChanged", callback)
}

/// Registers a callback for `gyroscopeStopped` event.
pub fn on_stopped(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("gyroscopeStopped", callback)
}

/// Registers a callback for `gyroscopeFailed` event.
pub fn on_failed(callback: &Closure<dyn Fn()>) -> Result<(), JsValue> {
    events::on_event("gyroscopeFailed", callback)
}

fn gyroscope_object() -> Result<JsValue, JsValue> {
    let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
    let tg = Reflect::get(&win, &"Telegram".into())?;
    let webapp = Reflect::get(&tg, &"WebApp".into())?;
    Reflect::get(&webapp, &"Gyroscope".into())
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
    fn setup_gyroscope() -> (Object, Object) {
        let win = window().unwrap();
        let telegram = Object::new();
        let webapp = Object::new();
        let gyro = Object::new();
        let _ = Reflect::set(&win, &"Telegram".into(), &telegram);
        let _ = Reflect::set(&telegram, &"WebApp".into(), &webapp);
        let _ = Reflect::set(&webapp, &"Gyroscope".into(), &gyro);
        (webapp, gyro)
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_ok() {
        let (_webapp, gyro) = setup_gyroscope();
        let func = Function::new_no_args("this.called = true;");
        let _ = Reflect::set(&gyro, &"start".into(), &func);
        assert!(start().is_ok());
        let called = Reflect::get(&gyro, &"called".into()).unwrap();
        assert_eq!(called.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn start_err() {
        let (_webapp, gyro) = setup_gyroscope();
        let _ = Reflect::set(&gyro, &"start".into(), &JsValue::from_f64(1.0));
        assert!(start().is_err());
    }

    #[wasm_bindgen_test]
    #[allow(clippy::unused_unit)]
    fn stop_ok() {
        let (_webapp, gyro) = setup_gyroscope();
        let func = Function::new_no_args("this.stopped = true;");
        let _ = Reflect::set(&gyro, &"stop".into(), &func);
        assert!(stop().is_ok());
        let stopped = Reflect::get(&gyro, &"stopped".into()).unwrap();
        assert_eq!(stopped.as_bool(), Some(true));
    }

    #[wasm_bindgen_test]
    fn get_angular_velocity_ok() {
        let (_webapp, gyro) = setup_gyroscope();
        let _ = Reflect::set(&gyro, &"x".into(), &JsValue::from_f64(0.1));
        let _ = Reflect::set(&gyro, &"y".into(), &JsValue::from_f64(0.2));
        let _ = Reflect::set(&gyro, &"z".into(), &JsValue::from_f64(0.3));
        let result = get_angular_velocity().unwrap();
        assert_eq!(
            result,
            AngularVelocity {
                x: 0.1,
                y: 0.2,
                z: 0.3
            }
        );
    }

    #[wasm_bindgen_test]
    fn registers_callbacks() {
        let (webapp, _gyro) = setup_gyroscope();
        let on_event = Function::new_with_args("name, cb", "this[name] = cb;");
        let _ = Reflect::set(&webapp, &"onEvent".into(), &on_event);
        let cb = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        on_started(&cb).expect("on_started");
        on_changed(&cb).expect("on_changed");
        on_stopped(&cb).expect("on_stopped");
        on_failed(&cb).expect("on_failed");
        assert!(Reflect::has(&webapp, &"gyroscopeStarted".into()).unwrap());
        assert!(Reflect::has(&webapp, &"gyroscopeChanged".into()).unwrap());
        assert!(Reflect::has(&webapp, &"gyroscopeStopped".into()).unwrap());
        assert!(Reflect::has(&webapp, &"gyroscopeFailed".into()).unwrap());
        cb.forget();
    }
}
