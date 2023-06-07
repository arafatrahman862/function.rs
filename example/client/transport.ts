import { RpcTransport } from "./lib/databuf.lib.ts";

export class HttpTransport implements RpcTransport {
    constructor(public url: URL | RequestInfo) { }
    unary() {
        let url = this.url;
        let chunks: Uint8Array[] = [];
        return {
            write(bytes: Uint8Array) {
                chunks.push(bytes)
            },
            flush() { },
            async call() {
                let body = concat_uint8(chunks);
                let res = await fetch(url, { method: "POST", body });
                if (!res.ok) {
                    throw new Error("Bad request");
                }
                return new Uint8Array(await res.arrayBuffer());
            }
        }
    }

    sse() {
        let url = this.url;
        let chunks: Uint8Array[] = [];
        return {
            write(bytes: Uint8Array) {
                chunks.push(bytes)
            },
            flush() { },
            async *call() {
                let body = concat_uint8(chunks);
                let res = await fetch(url, { method: "POST", body });
                if (!res.body) {
                    throw new Error("not expected empty body");
                }
                let reader = res.body.getReader();
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) return value
                    yield value
                }
            }
        }
    }

    async close() { }
}

function concat_uint8(chunks: Uint8Array[]) {
    if (chunks.length == 1) {
        return chunks[0]
    }
    let size = 0;
    for (const chunk of chunks) {
        size += chunk.byteLength;
    }
    const bytes = new Uint8Array(size);
    let offset = 0;
    for (const chunk of chunks) {
        bytes.set(chunk, offset);
        offset += chunk.byteLength;
    }
    return bytes;
}