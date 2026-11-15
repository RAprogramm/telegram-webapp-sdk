// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use js_sys::{Function, Promise, Reflect};
use serde::{Serialize, de::DeserializeOwned};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

/// Failure reported by a Telegram Serverless endpoint call.
///
/// Mirrors the `ServerlessError` object the JavaScript bridge passes to the
/// failure callback of `Telegram.WebApp.Serverless.call`. Client-side
/// failures (missing bridge, payload that cannot be (de)serialized) keep
/// [`Self::kind`] empty and [`Self::status`] at `0`.
#[derive(Clone, Debug, PartialEq)]
pub struct EndpointError {
    /// Human-readable description of the failure.
    pub message: String,

    /// Machine-readable error category reported by the endpoint, such as
    /// `"ENDPOINT_ERROR"` or `"UNAUTHORIZED"`; empty when Telegram did not
    /// classify the failure.
    pub kind: String,

    /// HTTP status code of the endpoint response, or `0` when the request
    /// could not be completed.
    pub status: u16,

    /// Additional structured data the endpoint passed along with its error.
    pub parameters: Option<JsValue>
}

impl EndpointError {
    /// Decodes a rejection value produced by the JavaScript bridge.
    ///
    /// Values that are not `ServerlessError` objects — for example the plain
    /// `Error` thrown when the app was not opened from Telegram — are mapped
    /// to an error with an empty [`Self::kind`] and a [`Self::status`] of `0`.
    #[must_use]
    pub fn from_js_value(value: &JsValue) -> Self {
        let read_string = |key: &str| {
            Reflect::get(value, &JsValue::from_str(key))
                .ok()
                .and_then(|field| field.as_string())
        };
        let message = read_string("message")
            .or_else(|| value.as_string())
            .unwrap_or_else(|| "unknown Serverless failure".to_string());
        let kind = read_string("type").unwrap_or_default();
        let status = Reflect::get(value, &JsValue::from_str("status"))
            .ok()
            .and_then(|field| field.as_f64())
            .and_then(|code| code.trunc().to_string().parse::<u16>().ok())
            .unwrap_or(0);
        let parameters = Reflect::get(value, &JsValue::from_str("parameters"))
            .ok()
            .filter(|field| !field.is_undefined() && !field.is_null());
        Self {
            message,
            kind,
            status,
            parameters
        }
    }

    const fn client_error(message: String) -> Self {
        Self {
            message,
            kind: String::new(),
            status: 0,
            parameters: None
        }
    }

    /// Returns an actionable hint describing how to recover from this error.
    ///
    /// The hint is keyed on the platform error category stored in
    /// [`Self::kind`] (see the Serverless section of the Bot API
    /// documentation) and falls back to the HTTP status class in
    /// [`Self::status`] when the failure was not classified.
    ///
    /// # Examples
    /// ```no_run
    /// use telegram_webapp_sdk::api::serverless::EndpointError;
    ///
    /// let error = EndpointError {
    ///     message:    "init data is invalid".to_string(),
    ///     kind:       "UNAUTHORIZED".to_string(),
    ///     status:     401,
    ///     parameters: None
    /// };
    /// assert!(error.hint().contains("initData"));
    /// ```
    #[must_use]
    pub fn hint(&self) -> &'static str {
        match self.kind.as_str() {
            "ENDPOINT_ERROR" => {
                "the endpoint refused the call; show `message` to the user and branch on \
                 machine-readable codes placed inside `parameters`"
            }
            "UNAUTHORIZED" => {
                "initData is missing or invalid; call init_sdk() before any Serverless call \
                 and reopen the Mini App link from the bot to obtain fresh initData"
            }
            "PAYMENT_REQUIRED" => {
                "the Mini App paid membership is missing or expired; ask the user to renew \
                 via Telegram.WebApp.openInvoice"
            }
            "FORBIDDEN" => "the user blocked the bot or access to this endpoint was revoked",
            "NOT_FOUND" => {
                "no endpoint with this name is deployed; check the name against the bot's \
                 Serverless configuration in BotFather"
            }
            "TOO_MANY_REQUESTS" => {
                "rate limited; back off and retry later, honoring retry_after in `parameters` \
                 when present"
            }
            _ if self.message.contains("InitDataUnavailable") => {
                "call init_sdk() before any Serverless call so the bridge can send initData"
            }
            _ if self.status == 429 => "rate limited; back off and retry later",
            _ if self.status >= 500 => {
                "the endpoint or the platform failed on Telegram's side; retry with \
                 exponential backoff"
            }
            _ if self.status >= 400 => {
                "the request was rejected; validate the input payload against the endpoint \
                 contract"
            }
            _ => {
                "the call did not reach the endpoint; run inside Telegram with network access \
                 and make sure Serverless is enabled for the bot in BotFather"
            }
        }
    }
}

impl std::fmt::Display for EndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "serverless call failed: {}", self.message)?;
        if !self.kind.is_empty() {
            write!(f, " [{}]", self.kind)?;
        }
        if self.status != 0 {
            write!(f, " (status {})", self.status)?;
        }
        write!(f, " — {}", self.hint())
    }
}

impl std::error::Error for EndpointError {}

impl From<EndpointError> for JsValue {
    fn from(error: EndpointError) -> Self {
        Self::from_str(&error.to_string())
    }
}

/// Calls `Telegram.WebApp.Serverless.call(name, input)` and resolves with the
/// raw value returned by the endpoint.
///
/// `input` is sent as the JSON body of the endpoint call; pass
/// [`JsValue::UNDEFINED`] when the endpoint takes no input. The returned
/// future fails with [`EndpointError`] both for synchronous bridge failures
/// (missing `Serverless` object, absent `initData`) and for endpoint errors
/// delivered through the JavaScript callback.
///
/// # Errors
/// Returns [`EndpointError`] if the bridge is unavailable, the endpoint
/// rejects the call, or the request cannot be completed.
///
/// # Examples
/// ```no_run
/// use telegram_webapp_sdk::api::serverless::call_raw;
/// use wasm_bindgen::JsValue;
/// # async fn run() -> Result<(), telegram_webapp_sdk::api::serverless::EndpointError> {
/// let result = call_raw("getProfile", &JsValue::UNDEFINED).await?;
/// # Ok(())
/// # }
/// ```
pub async fn call_raw(name: &str, input: &JsValue) -> Result<JsValue, EndpointError> {
    let win = window().ok_or_else(|| EndpointError::client_error("no window".to_string()))?;
    let serverless = Reflect::get(&win, &JsValue::from_str("Telegram"))
        .and_then(|telegram| Reflect::get(&telegram, &JsValue::from_str("WebApp")))
        .and_then(|webapp| Reflect::get(&webapp, &JsValue::from_str("Serverless")))
        .map_err(|error| EndpointError::from_js_value(&error))?;
    let call_fn = Reflect::get(&serverless, &JsValue::from_str("call"))
        .and_then(JsCast::dyn_into::<Function>)
        .map_err(|error| EndpointError::from_js_value(&error))?;
    let endpoint = JsValue::from_str(name);
    let input = if input.is_null() {
        JsValue::UNDEFINED
    } else {
        input.clone()
    };
    let promise = Promise::new(&mut |resolve, reject| {
        let callback = Closure::once_into_js({
            let reject = reject.clone();
            move |error: JsValue, result: JsValue| {
                if error.is_null() || error.is_undefined() {
                    let _ = resolve.call1(&JsValue::NULL, &result);
                } else {
                    let _ = reject.call1(&JsValue::NULL, &error);
                }
            }
        });
        if let Err(error) = call_fn.call3(&serverless, &endpoint, &input, &callback) {
            let _ = reject.call1(&JsValue::NULL, &error);
        }
    });
    JsFuture::from(promise)
        .await
        .map_err(|error| EndpointError::from_js_value(&error))
}

/// Calls a Serverless endpoint and deserializes its result.
///
/// `input` is serialized to a JavaScript object and sent as the endpoint
/// payload; pass `&()` for endpoints that take no input.
///
/// # Errors
/// Returns [`EndpointError`] if the bridge is unavailable, the endpoint
/// rejects the call, or the payload cannot be (de)serialized.
///
/// # Examples
/// ```no_run
/// use serde::{Deserialize, Serialize};
/// use telegram_webapp_sdk::api::serverless::call;
///
/// #[derive(Serialize)]
/// struct ProfileInput {
///     lang: String
/// }
///
/// #[derive(Deserialize)]
/// struct Profile {
///     name: String
/// }
///
/// # async fn run() -> Result<(), telegram_webapp_sdk::api::serverless::EndpointError> {
/// let profile: Profile = call(
///     "getProfile",
///     &ProfileInput {
///         lang: "en".into()
///     }
/// )
/// .await?;
/// # Ok(())
/// # }
/// ```
pub async fn call<I, O>(name: &str, input: &I) -> Result<O, EndpointError>
where
    I: Serialize + ?Sized,
    O: DeserializeOwned
{
    let payload = to_value(input).map_err(|error| {
        EndpointError::client_error(format!("failed to serialize input: {error}"))
    })?;
    let result = call_raw(name, &payload).await?;
    from_value(result).map_err(|error| {
        EndpointError::client_error(format!("failed to deserialize result: {error}"))
    })
}

#[cfg(test)]
mod tests {
    use js_sys::Object;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, PartialEq, serde::Serialize)]
    struct TypedInput {
        lang: String
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct TypedOutput {
        id: f64
    }

    fn install_serverless(body: &str) -> Result<Object, JsValue> {
        let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
        let telegram = Object::new();
        let webapp = Object::new();
        let serverless = Object::new();
        let func = Function::new_with_args("name, input, callback", body);
        Reflect::set(&serverless, &"call".into(), &func)?;
        Reflect::set(&webapp, &"Serverless".into(), &serverless)?;
        Reflect::set(&telegram, &"WebApp".into(), &webapp)?;
        Reflect::set(&win, &"Telegram".into(), &telegram)?;
        Ok(serverless)
    }

    #[wasm_bindgen_test(async)]
    async fn call_raw_resolves_with_endpoint_result() -> Result<(), JsValue> {
        let serverless = install_serverless("this.lastName = name; callback(null, { id: 42 });")?;
        let result = call_raw("getProfile", &JsValue::UNDEFINED).await?;
        assert_eq!(Reflect::get(&result, &"id".into())?.as_f64(), Some(42.0));
        assert_eq!(
            Reflect::get(&serverless, &"lastName".into())?.as_string(),
            Some("getProfile".to_string())
        );
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn call_raw_rejects_with_endpoint_error() -> Result<(), JsValue> {
        install_serverless(
            "var err = Error('boom'); err.name = 'ServerlessError'; err.status = 403; \
             err.type = 'UNAUTHORIZED'; err.parameters = { retryAfter: 5 }; callback(err, null);"
        )?;
        let error = call_raw("x", &JsValue::UNDEFINED)
            .await
            .expect_err("endpoint must fail");
        assert_eq!(error.message, "boom");
        assert_eq!(error.kind, "UNAUTHORIZED");
        assert_eq!(error.status, 403);
        assert!(error.parameters.is_some());
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn call_raw_maps_synchronous_throw() -> Result<(), JsValue> {
        install_serverless("throw Error('WebAppServerlessInitDataUnavailable');")?;
        let error = call_raw("x", &JsValue::UNDEFINED)
            .await
            .expect_err("throw must reject");
        assert_eq!(error.message, "WebAppServerlessInitDataUnavailable");
        assert_eq!(error.status, 0);
        assert_eq!(error.kind, "");
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn call_raw_fails_without_serverless_object() -> Result<(), JsValue> {
        let win = window().ok_or_else(|| JsValue::from_str("no window"))?;
        Reflect::set(&win, &"Telegram".into(), &JsValue::UNDEFINED)?;
        let error = call_raw("x", &JsValue::UNDEFINED)
            .await
            .expect_err("bridge must be missing");
        assert_eq!(error.status, 0);
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn call_round_trips_typed_payload() -> Result<(), JsValue> {
        install_serverless("callback(null, { id: input.lang === 'en' ? 7 : 0 });")?;
        let out: TypedOutput = call(
            "x",
            &TypedInput {
                lang: "en".into()
            }
        )
        .await?;
        assert_eq!(
            out,
            TypedOutput {
                id: 7.0
            }
        );
        Ok(())
    }

    #[wasm_bindgen_test(async)]
    async fn call_reports_deserialization_failure() -> Result<(), JsValue> {
        install_serverless("callback(null, 'not-an-object');")?;
        let result: Result<TypedOutput, _> = call("x", &()).await;
        let error = result.expect_err("shape mismatch must fail");
        assert!(error.message.contains("deserialize"));
        assert_eq!(error.status, 0);
        Ok(())
    }

    #[wasm_bindgen_test]
    fn endpoint_error_display_carries_kind_status_and_hint() {
        let error = EndpointError {
            message:    "boom".to_string(),
            kind:       "UNAUTHORIZED".to_string(),
            status:     401,
            parameters: None
        };
        let text = error.to_string();
        assert!(text.contains("serverless call failed: boom"));
        assert!(text.contains("[UNAUTHORIZED]"));
        assert!(text.contains("(status 401)"));
        assert!(text.contains("initData"));
    }

    #[wasm_bindgen_test]
    fn endpoint_error_hint_falls_back_to_status_class() {
        let unclassified = EndpointError {
            message:    "gateway exploded".to_string(),
            kind:       String::new(),
            status:     502,
            parameters: None
        };
        assert!(unclassified.hint().contains("backoff"));
        assert!(unclassified.to_string().contains("(status 502)"));

        let unreach = EndpointError {
            message:    "bridge missing".to_string(),
            kind:       String::new(),
            status:     0,
            parameters: None
        };
        assert!(unreach.hint().contains("BotFather"));
    }
}
