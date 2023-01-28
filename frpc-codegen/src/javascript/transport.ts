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

//----------------------------------------------------------

export interface ReadWriter extends Read, Write { }

export interface Read {
    read(bytes: Uint8Array): Promise<number>;
    /** Must not call this function more then once */
    close(): Promise<void>;
}

export interface Write {
    write(bytes: Uint8Array): Promise<number>
    write_all(bytes: Uint8Array): Promise<void>

    /** Must not call this function more then once */
    flush(): Promise<void>
}

export interface Transport {
    open_uni_stream(): Write;
    open_bi_stream(): ReadWriter;

    /** Must not call this function more then once */
    close(reason?: string): Promise<void>;
}

export type Result<T, E> =
    | { type: "Ok", value: T }
    | { type: "Err", value: E };
