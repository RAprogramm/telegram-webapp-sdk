// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Promise helpers shared by the modules that bridge callback-style
//! Telegram APIs into Rust futures.

use js_sys::{Function, Promise};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

/// Builds a `Promise` whose executor invokes `f` synchronously with the
/// `resolve` and `reject` callables. If `f` returns `Err`, the promise is
/// rejected with that value immediately. Used to wrap one-shot Telegram
/// callbacks into async-friendly futures.
pub(crate) fn one_shot_promise<F>(f: F) -> Promise
where
    F: FnOnce(Function, Function) -> Result<(), JsValue>
{
    let mut executor = Some(f);
    Promise::new(&mut |resolve, reject| {
        let Some(invoke) = executor.take() else {
            return;
        };
        if let Err(err) = invoke(resolve, reject.clone()) {
            let _ = reject.call1(&JsValue::NULL, &err);
        }
    })
}

/// Awaits a promise built by [`one_shot_promise`] and yields its value.
pub(crate) async fn await_one_shot(promise: Promise) -> Result<JsValue, JsValue> {
    JsFuture::from(promise).await
}
