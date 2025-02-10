// @ts-types="./pkg/wasm_http_hyper_example.d.ts"
import { parseBody } from "./pkg/wasm_http_hyper_example.js";
import http from "node:http";

const server = http.createServer(async (req, res) => {
  res.end(await parseBody(req));
});

const PORT = 3000;
server.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}/`);
  fetch("http://localhost:3000", {
    method: "POST",
    body: JSON.stringify({ username: "example" }),
  });
});
