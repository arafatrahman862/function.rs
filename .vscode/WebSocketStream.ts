import { deferred, Deferred } from "https://deno.land/std@0.158.0/async/mod.ts";

export interface Option {
    protocols?: string | string[]
}

export interface Connection {
    // deno-lint-ignore no-explicit-any
    readable: ReadableStream<MessageEvent<any>>
}

export class WebSocketStream {
    readonly ws: WebSocket;
    readonly connection: Deferred<Connection>
    readonly closed: Deferred<CloseEvent>

    constructor(url: string | URL, opt?: Option) {
        const ws = new WebSocket(url, opt?.protocols)
        const connection = deferred<Connection>()
        const closed = deferred<CloseEvent>()

        ws.onopen = () => connection.resolve({
            readable: new ReadableStream({
                start(controler) {
                    ws.onmessage = controler.enqueue
                    ws.onerror = controler.error
                    closed.then(controler.close)
                }
            }),
        })
        ws.onerror = connection.reject
        ws.onclose = closed.resolve

        this.ws = ws;
        this.connection = connection
        this.closed = closed
    }

    close(status?: { code?: number, reason?: string }) {
        this.ws.close(status?.code, status?.reason)
    }
}