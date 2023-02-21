export { Decoder } from "./decode.ts"
export { BufWriter } from "./encode.ts"

export type Result<T, E> =
    | { type: "Ok", value: T }
    | { type: "Err", value: E };

export type Option<T> = T | null;

export type NumSize<T extends "U" | "I" | "F"> =
    T extends "U" ? 1 | 2 | 4 | 8 | 16 :
    T extends "I" ? 1 | 2 | 4 | 8 | 16 :
    T extends "F" ? 4 | 8 : never;

export type Num<T extends "I" | "U" | "F", Size extends NumSize<T>> =
    T extends "U" | "I"
    ? (
        Size extends 1 | 2 | 4
        ? number
        : bigint
    )
    : T extends "F" ? number : never;

export interface Write {
    write(bytes: Uint8Array): void
    /** Must not call this function more then once */
    flush(): void
}

export function assertEq<T>(actual: T, expected: T) {
    if (!Object.is(actual, expected)) {
        throw new Error(`Assertion failed: expected ${expected}, but got ${actual}`);
    }
}

// deno-lint-ignore no-explicit-any
export function bytes_slice(buf: any, start = 0, end = buf.byteLength) {
    return new Uint8Array(buf.buffer, buf.byteOffset + start, end - start)
}