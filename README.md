# wasm-http-hyper

A Rust WebAssembly library that provides seamless integration between Node.js's `IncomingMessage` and Rust's `hyper::Request`. This library allows you to parse Node.js HTTP requests in your Rust WASM code using the familiar hyper interface.

## Features

- Convert Node.js `IncomingMessage` to `hyper::Request`
- Async body parsing with proper memory management
- Type-safe interfaces with TypeScript support
- Zero-copy buffer handling where possible

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wasm-http-hyper = "0.1.0"
```

## Usage

### Rust Code

```rust
use wasm_bindgen::prelude::*;
use wasm_http_hyper::IncomingMessage;

#[wasm_bindgen(js_name = "parseBody")]
pub async fn parse_body(incoming_message: IncomingMessage) -> String {
    let request = incoming_message.parse().await;
    String::from_utf8(request.body().to_vec()).unwrap_or_default()
}
```

### Node.js Code

```typescript
import { parseBody } from "./pkg/your_wasm_package.js";
import http from "node:http";

const server = http.createServer(async (req, res) => {
  res.end(await parseBody(req));
});

server.listen(3000, () => {
  console.log("Server running at http://localhost:3000/");
});
```

## How It Works

The library provides a bridge between Node.js's HTTP request handling and Rust's hyper ecosystem:

1. Takes a Node.js `IncomingMessage` object as input
2. Asynchronously reads the request body using Node.js streams
3. Converts headers, method, and URL to their hyper equivalents
4. Returns a fully-formed `hyper::Request` object

## Requirements

- Rust 1.75 or later
- wasm-bindgen 0.2 or later
- Node.js 16.0 or later

## Building

1. Install wasm-pack:
```bash
cargo install wasm-pack
```

2. Build the package:
```bash
wasm-pack build --target nodejs
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
