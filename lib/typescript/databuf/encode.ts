// deno-lint-ignore-file no-explicit-any prefer-const
import { Num, NumSize, Result, Write } from "./mod.ts";
import { bytes_slice, assertEq } from "./utils.ts";

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
    u32(num: number) {
        this.#unsafe_write(4, () => this.#view.setUint32(this.#written, num, true));
    }
    u64(num: bigint) {
        this.#unsafe_write(8, () => this.#view.setBigUint64(this.#written, num, true));
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
        return (num: Num<T, Size>) => {
            if (type == "F") {
                return (size == 4) ? this.f32(Number(num)) : this.f64(num as bigint)
            }
            let vars = [0x7f, 0x80, 7, 1, (size * 8) - 1];
            let [L7, MSB, n7, /* ZigZag shifter: */ l, m] = size >= 8 ? vars.map(BigInt) : vars as any;
            if (type == "I") {
                // Map integer with ZigZag Code
                (<any>num) = (num << l) ^ (num >> m)
            }
            let bytes = [];
            while (num > L7) {
                bytes.push(num | MSB);
                (<any>num) >>= n7;
            }
            bytes.push(num);
            bytes = size >= 8 ? bytes.map(Number) : bytes;
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


    u8_arr(len: number) {
        return (data: Uint8Array) => {
            assertEq(data.length, len);
            this.write_all(data);
        }
    }
    u16_arr(len: number) {
        return (data: Uint16Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }
    u32_arr(len: number) {
        return (data: Uint32Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }
    u64_arr(len: number) {
        return (data: BigUint64Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }

    i8_arr(len: number) {
        return (data: Int8Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }
    i16_arr(len: number) {
        return (data: Int16Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }
    i32_arr(len: number) {
        return (data: Int32Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
    }
    i64_arr(len: number) {
        return (data: BigInt64Array) => {
            assertEq(data.length, len);
            this.write_all(bytes_slice(data));
        };
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
        if (num < (1 << 7)) { this.u8(b2) }
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