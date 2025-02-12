use axum::extract::Request;
use axum::{routing::post, Router};
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Uint8Array;
use std::collections::HashMap;
use tower::util::ServiceExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use wasm_http_hyper::IncomingMessage;

wasm_bindgen_test_configure!(run_in_node_experimental);

// Import the mock IncomingMessage creation function from the other test file
#[wasm_bindgen(inline_js = r#"
import { EventEmitter } from 'events';
import { Readable } from 'stream';

export function createMockIncomingMessage(method, url, headers, bodyChunks) {
    const readable = new Readable({
        read() {
            for (const chunk of bodyChunks) {
                this.push(Buffer.from(chunk));
            }
            this.push(null);
        }
    });

    readable.method = method;
    readable.url = url;
    readable.headers = headers;

    return readable;
}
"#)]
extern "C" {
    fn createMockIncomingMessage(
        method: &str,
        url: &str,
        headers: &wasm_bindgen::JsValue,
        body_chunks: &wasm_bindgen::JsValue,
    ) -> IncomingMessage;
}

#[wasm_bindgen_test]
async fn test_axum_router() {
    // Create a simple Axum router
    let app = Router::new().route(
        "/test",
        post(|body: axum::extract::Json<serde_json::Value>| async move { axum::Json(body.0) }),
    );

    // Create mock request with JSON body
    let headers: HashMap<String, String> =
        [("content-type".to_string(), "application/json".to_string())]
            .into_iter()
            .collect();

    let headers_js = wasm_bindgen::JsValue::from_serde(&headers).unwrap();
    let body_chunks = js_sys::Array::new();

    // Create a JSON payload
    let json_data = r#"{"hello":"world"}"#;
    body_chunks.push(&Uint8Array::from(json_data.as_bytes()).buffer());

    let message = createMockIncomingMessage("POST", "/test", &headers_js, &body_chunks);

    // Convert IncomingMessage to axum Request
    let request: Request<axum::body::Body> = Request::from(&message);

    // Process the request through the router
    let response = app.oneshot(request).await.unwrap();

    // Verify the response
    assert_eq!(response.status(), 200);

    // Get the response body
    let body = response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();

    // Verify the JSON response matches what we sent
    assert_eq!(body_str, json_data);
}
