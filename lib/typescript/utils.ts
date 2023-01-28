import { WriteSync, ReadSync } from "./transport.ts";

// deno-lint-ignore no-explicit-any
export function bytes_slice(buf: any, start = 0, end = buf.byteLength) {
    return new Uint8Array(buf.buffer, buf.byteOffset + start, end - start)
}

export function write_all(self: WriteSync, buf: Uint8Array) {
    while (buf.byteLength > 0) {
        const amt = self.write(buf);
        if (amt == 0) {
            throw new Error("failed to write whole buffer")
        }
        buf = bytes_slice(buf, amt)
    }
}

export function read_exect(self: ReadSync, buf: Uint8Array) {
    while (buf.length > 0) {
        const amt = self.read(buf);
        if (amt == 0) {
            if (buf.length > 0) throw new Error("failed to fill whole buffer");
            return
        }
        buf = bytes_slice(buf, amt);
    }
}

export function char_from(bytes: Uint8Array) {
    return String.fromCharCode(
        (bytes[0] << 24) |
        (bytes[1] << 16) |
        (bytes[2] << 8) |
        bytes[3]
    );
}

