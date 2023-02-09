// deno-lint-ignore-file no-explicit-any
import { deferred, Deferred } from "https://deno.land/std@0.158.0/async/mod.ts";
import { ReadWriter, Transport, Write } from "./transport.ts";

export class Socket {
    #reader: ReadableStreamDefaultReader<MessageEvent<any>>;

    static async open(url: string | URL, opt: { protocols?: string | string[] } = {}) {
        const ws = new WebSocket(url, opt.protocols);
        const closed = deferred<CloseEvent>()

        const readableStream: ReadableStream = await new Promise((resolve, reject) => {
            ws.onerror = reject;
            ws.onopen = () => {
                resolve(new ReadableStream({
                    start(controler) {
                        ws.onmessage = (msg) => controler.enqueue(msg)
                        ws.onerror = (err) => controler.error(err)
                        ws.onclose = (ev) => {
                            closed.resolve(ev)
                            controler.close();
                        }
                    }
                }))
            }
        })
        return new Socket(ws, closed, readableStream);
    }

    constructor(protected ws: WebSocket, public readonly closed: Deferred<CloseEvent>, stream: ReadableStream) {
        this.#reader = stream.getReader();
        closed.then(() => {
            if (stream.locked) this.#reader.releaseLock()
        })
    }

    get bufferedAmount() {
        return this.ws.bufferedAmount
    }

    send(data: string | ArrayBufferLike | Blob | ArrayBufferView) {
        this.ws.send(data)
    }

    read() {
        return this.#reader.read()
    }

    async *[Symbol.asyncIterator]() {
        while (true) {
            const { done, value } = await this.read();
            if (done) return value
            yield value
        }
    }

    close(code?: number, reason?: string) {
        this.ws.close(code, reason)
    }
}

export class WebSocketTransport implements Transport {
    q = [];

    static async connect(url: string) {
        const socket = await Socket.open(url);
        return new WebSocketTransport(socket)
    }

    constructor(private socket: Socket) { }

    open_uni_stream(): Write {
        throw new Error("Method not implemented.");
    }

    open_bi_stream(): ReadWriter {
        throw new Error("Method not implemented.");
    }

    close(reason?: string | undefined): Promise<void> {
        throw new Error("Method not implemented.");
    }
}