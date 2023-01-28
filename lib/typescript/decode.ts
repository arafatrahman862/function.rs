import { Result } from "./transport.ts";
import { bytes_slice, char_from } from "./utils.ts";

type Decode<T> = (this: Decoder) => T;

export class Decoder {
    #offset = 0;
    #view: DataView;

    constructor(reader: Uint8Array) {
        this.#view = new DataView(reader.buffer, reader.byteOffset, reader.byteLength);
    }

    get offset() {
        return this.#offset
    }

    // -----------------------------------------------------------------------------------

    #unsafe_read<T>(amt: number, cb: () => T): T {
        const new_offset = this.#offset + amt;
        if (new_offset > this.#view.byteLength) {
            throw new Error("Insufficient bytes")
        }
        const num = cb.call(this);
        this.#offset = new_offset;
        return num
    }

    #read_slice(len: number) {
        return this.#unsafe_read(len, () => bytes_slice(this.#view, this.#offset, this.#offset + len))
    }

    // -----------------------------------------------------------------------------------

    u8() {
        return this.#unsafe_read(1, () => this.#view.getUint8(this.#offset))
    }
    u16() {
        return this.#unsafe_read(2, () => this.#view.getUint16(this.#offset, true))
    }
    u32() {
        return this.#unsafe_read(4, () => this.#view.getUint32(this.#offset, true))
    }
    u64() {
        return this.#unsafe_read(8, () => this.#view.getBigUint64(this.#offset, true))
    }

    i8() {
        return this.#unsafe_read(1, () => this.#view.getInt8(this.#offset))
    }
    i16() {
        return this.#unsafe_read(2, () => this.#view.getInt16(this.#offset, true))
    }
    i32() {
        return this.#unsafe_read(4, () => this.#view.getInt32(this.#offset, true))
    }
    i64() {
        return this.#unsafe_read(8, () => this.#view.getBigInt64(this.#offset, true))
    }

    f32() {
        return this.#unsafe_read(4, () => this.#view.getFloat32(this.#offset, true))
    }
    f64() {
        return this.#unsafe_read(8, () => this.#view.getFloat64(this.#offset, true))
    }

    // --------------------------------------------------------------------------------

    str() {
        const len = this.len_u22();
        const buf = this.#read_slice(len);
        return new TextDecoder().decode(buf);
    }

    char() {
        return char_from(this.#read_slice(4));
    }

    bool() {
        return !!this.u8()
    }

    u8_arr(len: number) { return new Uint8Array(this.#read_slice(len)) }
    u16_arr(len: number) { return new Uint16Array(this.#read_slice(len * 2)) }
    u32_arr(len: number) { return new Uint32Array(this.#read_slice(len * 4)) }
    u64_arr(len: number) { return new BigUint64Array(this.#read_slice(len * 8)) }

    i8_arr(len: number) { return new Int8Array(this.#read_slice(len)) }
    i16_arr(len: number) { return new Int16Array(this.#read_slice(len * 2)) }
    i32_arr(len: number) { return new Int32Array(this.#read_slice(len * 4)) }
    i64_arr(len: number) { return new BigInt64Array(this.#read_slice(len * 8)) }

    option<T>(v: Decode<T>) {
        return () => {
            if (this.u8() == 1) {
                return v.call(this)
            }
            return null
        }
    }

    result<T, E>(ok: Decode<T>, err: Decode<E>): () => Result<T, E> {
        return () => {
            if (this.u8() == 1) {
                return { type: "Ok", value: ok.call(this) }
            }
            return { type: "Err", value: err.call(this) }
        }
    }


    arr<T>(v: Decode<T>, len: number) {
        return () => {
            const values = []
            for (let i = 0; i < len; i++) {
                values.push(v.call(this))
            }
            return values
        }
    }

    vec<T>(v: Decode<T>) {
        const len = this.len_u22();
        return this.arr(v, len)
    }

    map<K, V>(k: Decode<K>, v: Decode<V>) {
        return () => {
            const map: Map<K, V> = new Map();
            const len = this.len_u22();
            for (let i = 0; i < len; i++) {
                const key = k.call(this);
                const value = v.call(this);
                map.set(key, value)
            }
            return map
        }
    }

    len_u15() {
        let num = this.u8();
        if (num >> 7 == 1) {
            const snd = this.u8();
            num = (num & 0x7F) | snd << 7; // num <- add 8 bits
        }
        return num
    }

    len_u22() {
        const num = this.u8();
        // if 1st bit is `0`
        if (num >> 7 == 0) { return num }
        // and 2nd bit is `0`
        else if (num >> 6 == 2) {
            const b2 = this.u8();
            return (num & 0x3F) | b2 << 6
        } else {
            // At this point, only possible first 2 bits are `11`
            const [b2, b3] = this.#read_slice(2);

            return (num & 0x3F)  // get last 6 bits
                | (b2) << 6     // add 8 bits from 2nd byte
                | (b3) << 14    // add 8 bits from 3rd byte
        }
    }
}
