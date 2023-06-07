#!/usr/bin/env -S deno run --allow-net="localhost" --unsafely-ignore-certificate-errors="localhost"

import { HttpTransport } from "../../lib/frpc-codegen/client/typescript/http.transport.ts";
import ValidateData from "../../target/rpc/ValidateData.ts";

let transport = new HttpTransport("https://localhost:4433/rpc");
let lib = new ValidateData(transport);

async function main() {
    await lib.user();
    let data = await lib.get_data();
    await lib.validate(data);
}

main().catch(console.error)


