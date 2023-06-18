#!/usr/bin/env -S deno run --allow-net="localhost" --unsafely-ignore-certificate-errors="localhost"

import { HttpTransport } from "../../libs/frpc-codegen/client/typescript/http.transport.ts";
import Lib from "../../target/rpc/ValidateTest.ts";

let lib = new Lib(new HttpTransport("https://localhost:4433/rpc/validate"));

async function main() {
    let data = await lib.get_data();
    await lib.validate(data);
}

main().catch(console.error)
