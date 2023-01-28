import { Result, WriteSync } from "./transport.ts";
import { bytes_slice, write_all } from "./utils.ts";

type Encode<T> = (this: BufWriter, value: T) => void;

export class BufWriter implements WriteSync {
    #inner: WriteSync;

    #written = 0;
    #view: DataView;

    constructor(writer: WriteSync, size = 4096) {
        this.#inner = writer;
        this.#view = new DataView(new ArrayBuffer(Math.max(size, 512)));
    }

    #write_buf() {
        write_all(this.#inner, new Uint8Array(this.#view.buffer, 0, this.#written));
        this.#written = 0;
    }

    #unsafe_write(bytes_len: number, cb: () => void) {
        if (bytes_len >= this.spare_capacity) {
            this.#write_buf();
        }
        cb.call(this)
        this.#written += bytes_len;
    }

    // --------------------------------------------------------------------------

    get spare_capacity() {
        return this.#view.byteLength - this.#written;
    }

    write(bytes: Uint8Array) {
        return this.#inner.write(bytes)
    }

    write_all(bytes: Uint8Array) {
        if (bytes.length >= this.spare_capacity) {
            this.#write_buf();
        }
        if (bytes.length >= this.#view.byteLength) {
            return write_all(this.#inner, bytes);
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
    u16(num: number) {
        this.#unsafe_write(2, () => this.#view.setUint16(this.#written, num, true));
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
    i16(num: number) {
        this.#unsafe_write(2, () => this.#view.setInt16(this.#written, num, true));
    }
    i32(num: number) {
        this.#unsafe_write(4, () => this.#view.setInt32(this.#written, num, true));
    }
    i64(num: bigint) {
        this.#unsafe_write(8, () => this.#view.setBigInt64(this.#written, num, true));
    }

    f32(num: number) {
        this.#unsafe_write(4, () => this.#view.setFloat32(this.#written, num, true));
    }
    f64(num: number) {
        this.#unsafe_write(8, () => this.#view.setFloat64(this.#written, num, true));
    }

    // -------------------------------------------------------------------------------------

    str(value: string) {
        const bytes = new TextEncoder().encode(value);
        this.len_u22(bytes.byteLength);
        this.write_all(bytes);
    }

    char(char: string) {
        const code = char.charCodeAt(0);
        const bytes = new Uint8Array(4);
        bytes[0] = (code & 0xff000000) >> 24;
        bytes[1] = (code & 0x00ff0000) >> 16;
        bytes[2] = (code & 0x0000ff00) >> 8;
        bytes[3] = code & 0x000000ff;
        this.write_all(bytes);
    }

    bool(ean: boolean) {
        this.u8(+ean)
    }

    u8_arr(bytes: Uint8Array) { this.write_all(bytes); }
    u16_arr(data: Uint16Array) { this.write_all(bytes_slice(data)); }
    u32_arr(data: Uint32Array) { this.write_all(bytes_slice(data)); }
    u64_arr(data: BigUint64Array) { this.write_all(bytes_slice(data)); }

    i8_arr(data: Int8Array) { this.write_all(bytes_slice(data)); }
    i16_arr(data: Int16Array) { this.write_all(bytes_slice(data)); }
    i32_arr(data: Int32Array) { this.write_all(bytes_slice(data)); }
    i64_arr(data: BigInt64Array) { this.write_all(bytes_slice(data)); }

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
            this.len_u22(values.length);
            this.arr(v)(values)
        }
    }

    map<K, V>(k: Encode<K>, v: Encode<V>) {
        return (values: Map<K, V>) => {
            this.len_u22(values.size);
            for (const [key, value] of values) {
                k.call(this, key);
                v.call(this, value);
            }
        }
    }

    len_u15(num: number) {
        let b1 = num;
        if (num < 128) { this.u8(b1) }
        else {
            b1 = 0x80 | b1; // 7 bits with MSB is set.
            const b2 = Uint8Array.of(num >> 7)[0]; // next 8 bits
            this.write_all(Uint8Array.from([b1, b2]))
        }
    }

    len_u22(num: number) {
        let b1 = num /* as u8 */;
        if (num < 128) { this.u8(b1) }
        else {
            b1 = b1 & 0x3F; // read last 6 bits
            const b2 = Uint8Array.of(num >> 6)[0]; // next 8 bits
            if (num < 0x4000) {
                // set first 2 bits  of `b1` = `10`
                this.write_all(Uint8Array.from([0x80 | b1, b2]))
            }
            else {
                // assert(num <= 0x3FFFFF);
                const b3 = Uint8Array.of(num >> 14)[0]; // next 8 bits
                // set first 2 bits  of `b1` to `11`
                this.write_all(Uint8Array.from([0xC0 | b1, b2, b3]))
            }
        }
    }
}
