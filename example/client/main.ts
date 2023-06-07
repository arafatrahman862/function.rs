#!/usr/bin/env -S deno run --allow-net="localhost" --unsafely-ignore-certificate-errors="localhost"

import { HttpTransport } from "./transport.ts";
import Example from "./lib/Example.ts";

let transport = new HttpTransport("https://localhost:4433/rpc");
let example = new Example(transport);

async function main() {
    let data = await example.get_data();
    await example.validate(data);
}

main().catch(console.error)
