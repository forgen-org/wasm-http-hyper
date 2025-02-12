use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use std::convert::Infallible;
use wasm_bindgen::JsValue;

use crate::IncomingMessage;

impl From<&IncomingMessage> for Request {
    fn from(message: &IncomingMessage) -> Self {
        let request: hyper::Request<BoxBody<Bytes, Infallible>> = message.into();
        let (parts, body) = request.into_parts();
        let mut builder = Request::builder().method(parts.method).uri(parts.uri);

        for (key, value) in parts.headers {
            if let Some(key) = key {
                builder = builder.header(key, value);
            }
        }
        builder
            .body(axum::body::Body::new(body))
            .expect("failed to build request")
    }
}

pub trait IntoJs {
    fn into_js(self) -> js_sys::Promise;
}

impl IntoJs for Response<Body> {
    fn into_js(self) -> js_sys::Promise {
        wasm_bindgen_futures::future_to_promise(async move {
            let body = self.into_body();
            let bytes = axum::body::to_bytes(body, 16384)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))
                .expect("failed to read body");
            Ok(JsValue::from_str(&String::from_utf8_lossy(&bytes)))
        })
    }
}
