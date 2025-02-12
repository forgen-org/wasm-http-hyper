use bytes::Bytes;
use futures::StreamExt;
use js_sys::{Function, Object, Uint8Array};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_node_experimental);

// // Mock IncomingMessage implementation for testing
// #[wasm_bindgen(inline_js = r#"
// import { EventEmitter } from 'events';
// import { Readable } from 'stream';

// export function createMockIncomingMessage(method, url, headers, bodyChunks) {
//     const readable = new Readable({
//         read() {
//             for (const chunk of bodyChunks) {
//                 this.push(Buffer.from(chunk));
//             }
//             this.push(null);
//         }
//     });

//     readable.method = method;
//     readable.url = url;
//     readable.headers = headers;

//     return readable;
// }
// "#)]
// extern "C" {
//     fn createMockIncomingMessage(
//         method: &str,
//         url: &str,
//         headers: &JsValue,
//         body_chunks: &JsValue,
//     ) -> crate::IncomingMessage;
// }

// #[wasm_bindgen_test]
// async fn test_incoming_message_basic_properties() {
//     let headers: HashMap<String, String> = [
//         ("content-type".to_string(), "application/json".to_string()),
//         ("x-test".to_string(), "value".to_string()),
//     ]
//     .into_iter()
//     .collect();

//     let headers_js = JsValue::from_serde(&headers).unwrap();
//     let body_chunks = js_sys::Array::new();
//     body_chunks.push(&Uint8Array::from(&[1, 2, 3][..]).buffer());
//     body_chunks.push(&Uint8Array::from(&[4, 5, 6][..]).buffer());

//     let message = createMockIncomingMessage("POST", "/test", &headers_js, &body_chunks);

//     assert_eq!(message.method(), "POST");
//     assert_eq!(message.url(), "/test");

//     let headers_map = message.headers_map();
//     assert_eq!(headers_map.get("content-type").unwrap(), "application/json");
//     assert_eq!(headers_map.get("x-test").unwrap(), "value");
// }

// #[wasm_bindgen_test]
// async fn test_incoming_message_body() {
//     let headers_js = JsValue::from_serde(&HashMap::<String, String>::new()).unwrap();
//     let body_chunks = js_sys::Array::new();
//     body_chunks.push(&Uint8Array::from(&[1, 2, 3][..]).buffer());
//     body_chunks.push(&Uint8Array::from(&[4, 5, 6][..]).buffer());

//     let message = createMockIncomingMessage("POST", "/test", &headers_js, &body_chunks);

//     // Test body_fut()
//     let body = message.body_fut().await;
//     assert_eq!(&body[..], &[1, 2, 3, 4, 5, 6]);

//     // Test body_stream()
//     let message = createMockIncomingMessage("POST", "/test", &headers_js, &body_chunks);

//     let chunks: Vec<Bytes> = message.body_stream().collect().await;
//     assert_eq!(chunks.len(), 2);
//     assert_eq!(&chunks[0][..], &[1, 2, 3]);
//     assert_eq!(&chunks[1][..], &[4, 5, 6]);
// }
