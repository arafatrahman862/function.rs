export interface Reader {
    read(bytes: Uint8Array): Promise<number>;
    read_exect(bytes: Uint8Array): Promise<number>;

    // Must not call this function more then once
    close(): Promise<void>;
}

export interface Writer {
    write(bytes: Uint8Array): Promise<number>
    write_all(bytes: Uint8Array): Promise<void>

    // Must not call this function more then once
    finish(): Promise<void>
}

export interface ReadWriter extends Reader, Writer { }
export interface Transport {
    // server does not return anything
    // call_and_forget(id: number, data: Uint8Array): Promise<undefined>;
    // unary request
    // call(id: number, data: Uint8Array): Promise<Uint8Array>;

    open_uni_stream(): Writer;
    open_bi_stream(): ReadWriter;

    close(reason?: string): Promise<void>;
}

export class Encoder {
    #writer: Writer
    constructor(writer: Writer) {
        this.#writer = writer;
    }
    u8(n: number) {
        // this.#writer.write()
    }
}

export module decode {

}

// export class Example {
//     #transport: Transport;

//     name(name: string, num: bigint) {
//         // let encoder = new Encoder();
//         // encoder.string(name)
//         // encoder.bigint(name)
//     }
// }