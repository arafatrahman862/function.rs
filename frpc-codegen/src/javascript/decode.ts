import { Result, ReadSync } from "./transport";
import { char_from, read_exect } from "./utils";

type Decode<T> = (this: Decoder, value: T) => void;

export class Decoder implements ReadSync {
    #inner: ReadSync;

    constructor(reader: ReadSync) {
        this.#inner = reader;
    }

    read(bytes: Uint8Array): number {
        return this.#inner.read(bytes)
    }

    read_exect(bytes: Uint8Array) {
        return read_exect(this.#inner, bytes);
    }

    close(): void {
        return this.#inner.close()
    }

    // --------------------------------------------------------------------------------------

    u8(): number {
        // this.read_exect()
        return 0
    }

    u8_arr(buf: Uint8Array) {
        this.read_exect(buf)
    }

    *arr<T>(v: Decode<T>, len: number) {
        for (let i = 0; i < len; i++) {
            yield v.call(this)
        }
    }

    len_u15() {
        let num = this.u8();
        if (num >> 7 == 1) {
            let snd = this.u8();
            num = (num & 0x7F) | snd << 7; // num <- add 8 bits
        }
        return num
    }

    len_u22() {
        let num = this.u8();
        // if 1st bit is `0`
        if (num >> 7 == 0) { return num }
        // and 2nd bit is `0`
        else if (num >> 6 == 2) {
            let b2 = this.u8();
            return (num & 0x3F) | b2 << 6
        } else {
            // At this point, only possible first 2 bits are `11`
            let buf = new Uint8Array(2);
            this.read_exect(buf);

            return (num & 0x3F)  // get last 6 bits
                | (buf[0]) << 6     // add 8 bits from 2nd byte
                | (buf[1]) << 14    // add 8 bits from 3rd byte
        };
    }

    char() {
        let bytes = new Uint8Array(4);
        this.read_exect(bytes);
        return char_from(bytes);
    }
}
