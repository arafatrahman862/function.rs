export module use {
	export type Option<T> = T | null;
	export type Result<T, E> = | { type: "Ok", value: T } | { type: "Err", value: E };

	export type Num<T extends "I" | "U", Size extends NumSize<T>> = Size extends 16 | 32 ? number : bigint;

	export type NumSize<T extends "U" | "I"> =
		T extends "U" ? 16 | 32 | 64 | 128 :
		T extends "I" ? 16 | 32 | 64 | 128 :
		never;

	export interface Write {
		write(bytes: Uint8Array): void
		/** Must not call this function more then once */
		flush(): void
	}

	export interface RpcTransport {
		unary(): Write & { call(): Promise<Uint8Array> }
		sse(): Write & { call(): AsyncGenerator<Uint8Array> }
		close(): Promise<void>
	}

	function assertEq<T>(actual: T, expected: T) {
		if (!Object.is(actual, expected)) {
			throw new Error(`Assertion failed: expected ${expected}, but got ${actual}`);
		}
	}

	function bytes_slice(buf: any, start = 0, end = buf.byteLength) {
		return new Uint8Array(buf.buffer, buf.byteOffset + start, end - start)
	}

	export type Decode<T> = (this: Decoder) => T;
	export class Decoder {
		#offset = 0;
		#view: DataView;

		constructor(slice: Uint8Array) {
			this.#view = new DataView(slice.buffer, slice.byteOffset, slice.byteLength);
		}

		get offset() { return this.#offset }

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

		u8() { return this.#unsafe_read(1, () => this.#view.getUint8(this.#offset)) }
		i8() { return this.#unsafe_read(1, () => this.#view.getInt8(this.#offset)) }
		f32() { return this.#unsafe_read(4, () => this.#view.getFloat32(this.#offset, true)) }
		f64() { return this.#unsafe_read(8, () => this.#view.getFloat64(this.#offset, true)) }

		num<T extends "I" | "U", Size extends NumSize<T>>(type: T, size: Size) {
			return () => {
				let num = 0n;
				let shift = 0n;
				while (true) {
					let byte = this.u8();
					num |= BigInt((byte & 0x7F)) << shift;
					if ((byte & 0x80) == 0) {
						let bint = type == "I" ? (num >> 1n) ^ -(num & 1n) : num;
						return ((size >= 64) ? bint : Number(bint)) as Num<T, Size>
					}
					shift += 7n;
				}
			}
		}

		bool() { return !!this.u8() }
		char() { return String.fromCharCode(this.num("U", 32)()); }
		str() {
			let len = this.len_u30();
			let buf = this.#read_slice(len);
			return new TextDecoder().decode(buf);
		}

		option<T>(v: Decode<T>) {
			return () => {
				if (this.bool()) {
					return v.call(this)
				}
				return null
			}
		}

		result<T, E>(ok: Decode<T>, err: Decode<E>): () => Result<T, E> {
			return () => {
				if (this.bool()) {
					return { type: "Ok", value: ok.call(this) }
				}
				return { type: "Err", value: err.call(this) }
			}
		}

		arr<T>(v: Decode<T>, len: number) {
			return () => {
				let values: T[] = [];
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

	type Encode<T> = (this: BufWriter, value: T) => void;

	export class BufWriter implements Write {
		#written = 0;
		#inner: Write;
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

		get spareCapacity() { return this.#view.byteLength - this.#written; }

		write(bytes: Uint8Array) {
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

		u16(num: number) { this.#unsafe_write(2, () => this.#view.setUint16(this.#written, num, true)); }

		u8(num: number) { this.#unsafe_write(1, () => this.#view.setUint8(this.#written, num)); }
		i8(num: number) { this.#unsafe_write(1, () => this.#view.setInt8(this.#written, num)); }
		f32(num: number) { this.#unsafe_write(4, () => this.#view.setFloat32(this.#written, num, true)); }
		f64(num: number) { this.#unsafe_write(8, () => this.#view.setFloat64(this.#written, num, true)); }

		num<T extends "I" | "U", Size extends NumSize<T>>(type: T, size: Size) {
			let bits = BigInt(size);
			let max = (1n << bits) - 1n;
			return (num: Num<T, Size>) => {
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

		bool(bool: boolean) { this.u8(+bool) }
		char(char: string) { this.num("U", 32)(char.charCodeAt(0)) }

		str(value: string) {
			const bytes = new TextEncoder().encode(value);
			this.len_u30(bytes.byteLength);
			this.write(bytes);
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
				for (const value of values)
					v.call(this, value);
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
				for (let i = 0; i < encoders.length; i++)
					encoders[i].call(this, values[i]);
			}
		}

		len_u15(num: number) {
			let b2 = num;
			if (num < (1 << 7)) { return this.u8(b2) }
			if (num < (1 << 15)) {
				let b1 = (num >> 8) & 0xFF;
				return this.write(Uint8Array.from([0x80 | b1, b2]))
			}
			throw new Error("out of range integral type conversion attempted")
		}

		len_u30(num: number) {
			let b4 = num & 0xff;
			if (num < (1 << 6)) { return this.write(Uint8Array.from([b4])); }
			let b3 = (num >> 8) & 0xff;
			if (num < (1 << 14)) { return this.write(Uint8Array.from([0x40 | b3, b4])); }
			let b2 = (num >> 16) & 0xff;
			if (num < (1 << 22)) { return this.write(Uint8Array.from([0x80 | b2, b3, b4])); }
			let b1 = (num >> 24) & 0xff;
			if (num < (1 << 30)) { return this.write(Uint8Array.from([0xC0 | b1, b2, b3, b4])) }
			throw new Error("out of range integral type conversion attempted")
		}
	}
}
