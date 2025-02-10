import { Effect, Layer } from "effect";
import {
  HttpRouter,
  HttpServer,
  HttpServerRequest,
  HttpServerResponse,
} from "@effect/platform";
import { NodeHttpServer, NodeRuntime } from "@effect/platform-node";
import { parseBody } from "./pkg/wasm_http_hyper_example.js";
import { createServer } from "node:http";

const router = HttpRouter.empty.pipe(
  HttpRouter.all(
    "*",
    Effect.gen(function* () {
      const request = yield* HttpServerRequest.HttpServerRequest;
      const body = yield* Effect.tryPromise({
        try: () => parseBody(request.source),
        catch: (error) => Effect.fail(error),
      });
      return HttpServerResponse.raw(body);
    }),
  ),
);

const app = router.pipe(
  HttpServer.serve(),
  HttpServer.withLogAddress,
);

NodeRuntime.runMain(
  Layer.launch(
    app.pipe(
      Layer.provide(
        NodeHttpServer.layer(() => createServer(), { port: 3000 }),
      ),
    ),
  ),
);
