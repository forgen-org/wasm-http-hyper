use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use gloo_utils::format::JsValueSerdeExt;
use std::{collections::HashMap, future::Future};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_IMPORT: &'static str = r#"
import { IncomingMessage } from "node:http";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IncomingMessage")]
    pub type IncomingMessage;

    #[wasm_bindgen(method)]
    pub fn on(this: &IncomingMessage, event: &str, callback: &js_sys::Function) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn method(this: &IncomingMessage) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn url(this: &IncomingMessage) -> String;

    #[wasm_bindgen(method, getter, js_name = headers)]
    pub fn headers(this: &IncomingMessage) -> js_sys::Object;
}

impl IncomingMessage {
    pub fn body_fut(&self) -> impl Future<Output = Bytes> {
        let body = self.body_stream();
        async move {
            let bytes: Vec<_> = body.collect().await;
            let mut result = Vec::new();
            for chunk in bytes {
                result.extend_from_slice(&chunk);
            }
            Bytes::from(result)
        }
    }

    pub fn body_stream(&self) -> impl Stream<Item = Bytes> {
        let (tx, rx) = futures::channel::mpsc::unbounded();

        let data_handler = Closure::wrap(Box::new({
            let tx = tx.clone();
            move |chunk: JsValue| {
                let buffer = js_sys::Uint8Array::new(&chunk).to_vec();
                let _ = tx.unbounded_send(Bytes::from(buffer));
            }
        }) as Box<dyn FnMut(JsValue)>);
        let end_handler = Closure::wrap(Box::new(move || {
            let _ = tx.close_channel();
        }) as Box<dyn FnMut()>);

        self.on("data", data_handler.as_ref().unchecked_ref());
        self.on("end", end_handler.as_ref().unchecked_ref());

        // Prevent handlers from being dropped
        data_handler.forget();
        end_handler.forget();

        rx
    }

    pub fn headers_map(&self) -> HashMap<String, String> {
        self.headers()
            .into_serde::<HashMap<String, String>>()
            .expect("headers is not a map")
    }
}
