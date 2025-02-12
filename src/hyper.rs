use std::{convert::Infallible, str::FromStr};

use bytes::Bytes;
use futures::StreamExt;
use http_body_util::{combinators::BoxBody, StreamBody};
use hyper::{body::Frame, Method, Request};

use crate::IncomingMessage;

impl From<&IncomingMessage> for Request<BoxBody<Bytes, Infallible>> {
    fn from(message: &IncomingMessage) -> Self {
        let body = message.body_stream();
        let stream = body.map(|chunk| Ok::<_, Infallible>(Frame::data(chunk)));
        let stream = StreamBody::new(stream);
        let body: BoxBody<Bytes, Infallible> = BoxBody::new(stream);

        let headers = message.headers_map();
        let method = message.method();
        let url = message.url();

        let mut request_builder = Request::builder()
            .method(Method::from_str(&method).expect("failed to parse method"))
            .uri(url);
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        let request = request_builder.body(body).expect("failed to build request");
        request
    }
}
