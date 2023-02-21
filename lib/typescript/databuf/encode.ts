// deno-lint-ignore-file no-explicit-any prefer-const
import { Num, NumSize, Result, Write } from "./mod.ts";

type Encode<T> = (this: BufWriter, value: T) => void;

export class BufWriter implements Write {
    #inner: Write;

    #written = 0;
    #view: DataView;

    constructor(writer: Write, size = 4096) {
        this.#inner = writer;
        this.#view = new DataView(new ArrayBuffer(Math.max(size, 512)));
    }

    #write_buf() {
        this.#inner.write(new Uint8Array(this.#view.buffer, 0, this.#written));
        this.#written = 0;
    }

    #unsafe_write(bytes_len: number, cb: () => void) {
        if (bytes_len >= this.spareCapacity) {
            this.#write_buf();
        }
        cb.call(this)
        this.#written += bytes_len;
    }

    // --------------------------------------------------------------------------

    get spareCapacity() {
        return this.#view.byteLength - this.#written;
    }

    write(bytes: Uint8Array) {
        return this.#inner.write(bytes)
    }

    write_all(bytes: Uint8Array) {
        if (bytes.length >= this.spareCapacity) {
            this.#write_buf();
        }
        if (bytes.length >= this.#view.byteLength) {
            return this.#inner.write(bytes);
        }
        new Uint8Array(this.#view.buffer).set(bytes, this.#written);
        this.#written += bytes.length;
    }

    flush() {
        this.#write_buf();
        this.#inner.flush();
    }

    // --------------------------------------------------------------------------

    u8(num: number) {
        this.#unsafe_write(1, () => this.#view.setUint8(this.#written, num));
    }
    i8(num: number) {
        this.#unsafe_write(1, () => this.#view.setInt8(this.#written, num));
    }
    f32(num: number) {
        this.#unsafe_write(4, () => this.#view.setFloat32(this.#written, num, true));
    }
    f64(num: number) {
        this.#unsafe_write(8, () => this.#view.setFloat64(this.#written, num, true));
    }
    num<T extends "I" | "U" | "F", Size extends NumSize<T>>(type: T, size: Size) {
        let bits = BigInt(size * 8);
        let max = (1n << bits) - 1n;
        return (num: Num<T, Size>) => {
            if (size == 1) {
                return type == "U" ? this.u8(num as number) : this.i8(num as number);
            }
            if (type == "F") {
                return (size == 4) ? this.f32(num as number) : this.f64(num as number)
            }
            let int = BigInt(num);
            if (type == "I") {
                // Map integer with ZigZag Code
                int = (int << 1n) ^ (int >> bits - 1n)
            }
            if (int > max) {
                throw new Error(`Max value: ${max}, But got: ${int}`)
            }
            while (int > 0b111_1111n) {
                this.u8(Number((int & 0xffn) | 0x80n));
                int >>= 7n;
            }
            this.u8(Number(int));
        }
    }

    // -------------------------------------------------------------------------------------

    str(value: string) {
        const bytes = new TextEncoder().encode(value);
        this.len_u30(bytes.byteLength);
        this.write_all(bytes);
    }

    char(char: string) {
        this.num("U", 4)(char.charCodeAt(0))
    }

    bool(bool: boolean) {
        this.u8(+bool)
    }

    option<T>(v: Encode<T>) {
        return (value: null | T) => {
            if (value === null) {
                this.u8(0);
            } else {
                this.u8(1);
                v.call(this, value)
            }
        }
    }

    result<T, E>(ok: Encode<T>, err: Encode<E>) {
        return ({ type, value }: Result<T, E>) => {
            if (type == "Ok") {
                this.u8(1);
                ok.call(this, value);
            } else {
                this.u8(0);
                err.call(this, value);
            }
        }
    }

    arr<T>(v: Encode<T>) {
        return (values: Array<T>) => {
            for (const value of values) {
                v.call(this, value)
            }
        }
    }

    vec<T>(v: Encode<T>) {
        return (values: T[]) => {
            this.len_u30(values.length);
            this.arr(v)(values)
        }
    }

    map<K, V>(k: Encode<K>, v: Encode<V>) {
        return (values: Map<K, V>) => {
            this.len_u30(values.size);
            for (const [key, value] of values) {
                k.call(this, key);
                v.call(this, value);
            }
        }
    }

    tuple<Encoders extends Encode<any>[]>(...encoders: Encoders) {
        return (values: { [K in keyof Encoders]: Parameters<Encoders[K]>[0] }) => {
            for (let i = 0; i < encoders.length; i++) {
                encoders[i].call(this, values[i])
            }
        }
    }

    len_u15(num: number) {
        let b2 = num;
        if (num < (1 << 7)) { return this.u8(b2) }
        if (num < (1 << 15)) {
            let b1 = (num >> 8) & 0xFF;
            return this.write_all(Uint8Array.from([0x80 | b1, b2]))
        }
        throw new Error("out of range integral type conversion attempted")
    }

    len_u30(num: number) {
        let b4 = num & 0xff;
        if (num < (1 << 6)) {
            return this.write_all(Uint8Array.from([b4]));
        }
        let b3 = (num >> 8) & 0xff;
        if (num < (1 << 14)) {
            return this.write_all(Uint8Array.from([0x40 | b3, b4]));
        }
        let b2 = (num >> 16) & 0xff;
        if (num < (1 << 22)) {
            return this.write_all(Uint8Array.from([0x80 | b2, b3, b4]));
        }
        let b1 = (num >> 24) & 0xff;
        if (num < (1 << 30)) {
            return this.write_all(Uint8Array.from([0xC0 | b1, b2, b3, b4]))
        }
        throw new Error("out of range integral type conversion attempted")
    }
}