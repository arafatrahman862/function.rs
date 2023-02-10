// export interface ReadWriter extends Read, Write { }

import { Write } from "./databuf/mod.ts"

// export interface Read {
//     read(bytes: Uint8Array): Promise<number>;
// }

// export interface Write {
//     write_all(bytes: Uint8Array): Promise<void>
//     /** Must not call this function more then once */
//     flush(): Promise<void>
// }

// export interface Transport {
//     open_uni_stream(): Write;
//     open_bi_stream(): ReadWriter;
//     /** Must not call this function more then once */
//     close(reason?: string): Promise<void>;
// }

export interface RPC {
    unary_call(): Write & { output(): Promise<ArrayBuffer> }
    close(): void
}