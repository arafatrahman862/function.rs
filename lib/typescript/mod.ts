export interface Write {
    write(bytes: Uint8Array): void
    flush(): void
}

export interface RPC {
    unary(): Write & { call(): Promise<ArrayBuffer> }
    sse(): Write & { call(): AsyncGenerator<Uint8Array> }
    close(): void
}

export class HttpTransport implements RPC {
    constructor(public url: URL | RequestInfo) { }
    unary(): Write & { call(): Promise<ArrayBuffer> } {
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
                return res.arrayBuffer();
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

    close() { }
}

function concat_uint8(chunks: Uint8Array[]) {
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