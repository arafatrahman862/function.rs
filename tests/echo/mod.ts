#!/usr/bin/env -S deno run --allow-net="localhost" --unsafely-ignore-certificate-errors="localhost"

import { HttpTransport } from "../../lib/frpc-codegen/client/typescript/http.transport.ts";
import Lib from "../../target/rpc/EchoTest.ts";

let lib = new Lib(new HttpTransport("https://localhost:4433/rpc"));

async function main() {
    await lib.log(true);
    // let data = await lib.log
}

main().catch(console.error)
