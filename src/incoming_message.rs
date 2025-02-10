use std::{collections::HashMap, rc::Rc, str::FromStr, sync::Mutex};

use gloo_utils::format::JsValueSerdeExt;
use hyper::{Method, Request};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_IMPORT: &'static str = r#"
import { IncomingMessage } from "node:http";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IncomingMessage")]
    pub type IncomingMessage;
}

impl IncomingMessage {
    pub async fn parse(&self) -> Request<Vec<u8>> {
        let js_value: JsValue = self.into();

        let body = self.parse_body().await;
        let headers = js_sys::Reflect::get(&js_value, &JsValue::from_str("headers"))
            .expect("headers is not a map")
            .into_serde::<HashMap<String, String>>()
            .expect("headers is not a map");
        let method = js_sys::Reflect::get(&js_value, &JsValue::from_str("method"))
            .expect("method is not a string")
            .as_string()
            .expect("method is not a string");
        let uri = js_sys::Reflect::get(&js_value, &JsValue::from_str("url"))
            .expect("url is not a string")
            .as_string()
            .expect("url is not a string");

        let mut request_builder = Request::builder()
            .method(Method::from_str(&method).expect("failed to parse method"))
            .uri(uri);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
        let request = request_builder.body(body).expect("failed to build request");
        request
    }

    async fn parse_body(&self) -> Vec<u8> {
        let js_value: JsValue = self.into();
        let on = js_sys::Reflect::get(&js_value, &JsValue::from_str("on"))
            .expect("on is not a function");
        let on_fn = js_sys::Function::from(on);

        let body = Rc::new(Mutex::new(Vec::new()));
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let data_handler = Closure::wrap(Box::new({
                let body = body.clone();
                move |chunk: JsValue| {
                    let buffer = js_sys::Uint8Array::new(&chunk).to_vec();
                    body.lock().unwrap().extend(buffer);
                }
            }) as Box<dyn FnMut(JsValue)>);

            let end_handler = Closure::wrap(Box::new({
                move || {
                    resolve
                        .call0(&JsValue::NULL)
                        .expect("failed to call resolve");
                }
            }) as Box<dyn FnMut()>);

            on_fn
                .call2(
                    &js_value,
                    &JsValue::from_str("data"),
                    &data_handler.as_ref().unchecked_ref(),
                )
                .expect("failed to call data");

            on_fn
                .call2(
                    &js_value,
                    &JsValue::from_str("end"),
                    &end_handler.as_ref().unchecked_ref(),
                )
                .expect("failed to call end");

            // Prevent handlers from being dropped
            data_handler.forget();
            end_handler.forget();
        });

        // Wait for all data to be received
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .expect("failed to wait for promise");

        let body = body.lock().expect("failed to lock body").clone();
        body
    }
}
