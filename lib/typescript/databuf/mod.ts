export { Decoder } from "./decode.ts"
export { BufWriter } from "./encode.ts"

export type Result<T, E> =
    | { type: "Ok", value: T }
    | { type: "Err", value: E };

export type Option<T> = T | null;

export interface Write {
    write(bytes: Uint8Array): void
    /** Must not call this function more then once */
    flush(): void
}
