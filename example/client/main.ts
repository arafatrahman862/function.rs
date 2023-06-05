import { use } from "./lib/databuf.lib.ts";
import { HttpTransport } from "./transport.ts";
let { log: println } = console;

let transport = new HttpTransport("https://localhost:4433/rpc");

async function main() {
    let fn = transport.unary();
    const d = new use.BufWriter(fn);
    d.u16(1);
    d.num('I', 32)(10);
    d.num('I', 32)(20);
    d.flush();
    let f = await fn.call()
        .then(buf => new use.Decoder(buf))
        .then(d => d.num('I', 32)());

    console.log(f);
    // let s = await fn.call();
    // println(s);
}

main().catch(println)