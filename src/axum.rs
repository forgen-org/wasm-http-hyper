use axum::extract::Request;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use std::convert::Infallible;

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
