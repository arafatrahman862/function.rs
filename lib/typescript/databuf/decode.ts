// deno-lint-ignore-file no-explicit-any prefer-const
import { Result, Num, NumSize, bytes_slice } from "./mod.ts";

export type Decode<T> = (this: Decoder) => T;

export class Decoder {
    #offset = 0;
    #view: DataView;

    constructor(slice: Uint8Array) {
        this.#view = new DataView(slice.buffer, slice.byteOffset, slice.byteLength);
    }

    get offset() {
        return this.#offset
    }

    // -----------------------------------------------------------------------------------

    #unsafe_read<T>(amt: number, cb: () => T): T {
        let new_offset = this.#offset + amt;
        if (new_offset > this.#view.byteLength) {
            throw new Error("Insufficient bytes")
        }
        let num = cb.call(this);
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
    i8() {
        return this.#unsafe_read(1, () => this.#view.getInt8(this.#offset))
    }

    f32() {
        return this.#unsafe_read(4, () => this.#view.getFloat32(this.#offset, true))
    }
    f64() {
        return this.#unsafe_read(8, () => this.#view.getFloat64(this.#offset, true))
    }

    num<T extends "I" | "U" | "F", Size extends NumSize<T>>(type: T, size: Size) {
        return () => {
            if (size == 1) {
                return (type == "U" ? this.u8() : this.i8()) as Num<T, Size>;
            }
            if (type == "F") {
                return ((size == 4) ? this.f32() : this.f64()) as Num<T, Size>
            }
            let num = 0n;
            let shift = 0n;
            while (true) {
                let byte = this.u8();
                num |= BigInt((byte & 0x7F)) << shift;
                if ((byte & 0x80) == 0) {
                    let bint = type == "I" ? (num >> 1n) ^ -(num & 1n) : num;
                    return ((size >= 8) ? bint : Number(bint)) as Num<T, Size>
                }
                shift += 7n;
            }
        }
    }

    // --------------------------------------------------------------------------------

    str() {
        let len = this.len_u30();
        let buf = this.#read_slice(len);
        return new TextDecoder().decode(buf);
    }

    char() {
        return String.fromCharCode(this.num("U", 4)());
    }

    bool() {
        return !!this.u8()
    }

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
            let values = []
            for (let i = 0; i < len; i++) {
                values.push(v.call(this))
            }
            return values
        }
    }

    vec<T>(v: Decode<T>) {
        return () => {
            let len = this.len_u30();
            return this.arr(v, len)()
        }
    }

    map<K, V>(k: Decode<K>, v: Decode<V>) {
        return () => {
            let map: Map<K, V> = new Map();
            let len = this.len_u30();
            for (let i = 0; i < len; i++) {
                let key = k.call(this);
                let value = v.call(this);
                map.set(key, value)
            }
            return map
        }
    }

    tuple<T extends Decode<any>[]>(...args: T) {
        return () => {
            let tuples = [] as { [K in keyof T]: ReturnType<T[K]> };
            for (let arg of args) {
                tuples.push(arg.call(this))
            }
            return tuples
        }
    }

    len_u15() {
        let b1 = this.u8();
        if (b1 >> 7 == 0) {
            return b1
        }
        let b2 = this.u8();
        return ((b1 & 0x7F) << 8) | b2
    }

    len_u30() {
        let num = this.u8();
        let len = num >> 6;
        num &= 0x3F;
        for (let i = 0; i < len; i++) {
            num = (num << 8) + this.u8()
        }
        return num
    }
}