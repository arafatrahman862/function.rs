#!/usr/bin/env -S deno run --allow-net=localhost --unsafely-ignore-certificate-errors=localhost

import { HttpTransport } from "../../lib/frpc-codegen/client/typescript/http.transport.ts";
import Lib, { Echo_Log as Log } from "../../target/rpc/EchoTest.ts";

let lib = new Lib(new HttpTransport("https://localhost:4433/rpc/echo"));

await lib.log(Log.Enable);

var f = await lib.echo_f32(5);
var f = await lib.echo_f32(5);
console.log(f);
console.clear();