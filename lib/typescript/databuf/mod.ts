export { Decoder } from "./decode.ts"
export { BufWriter } from "./encode.ts"

export type Result<T, E> =
    | { type: "Ok", value: T }
    | { type: "Err", value: E };

export type Option<T> = T | null;

export interface ReadWriterSync extends ReadSync, WriteSync { }

export interface ReadSync {
    read(bytes: Uint8Array): number;
    /** Must not call this function more then once */
    close(): void;
}

export interface WriteSync {
    write(bytes: Uint8Array): number
    /** Must not call this function more then once */
    flush(): void
}
