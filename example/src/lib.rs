use wasm_bindgen::prelude::*;
use wasm_http_hyper::IncomingMessage;

#[wasm_bindgen(js_name = "parseBody")]
pub async fn parse_body(incoming_message: IncomingMessage) -> String {
    let request = incoming_message.parse().await;
    String::from_utf8(request.body().to_vec()).unwrap_or_default()
}
