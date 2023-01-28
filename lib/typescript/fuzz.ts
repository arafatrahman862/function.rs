const r = crypto.getRandomValues.bind(crypto);


export const rand = {
    /** https://stackoverflow.com/questions/1349404/generate-random-string-characters-in-javascript */
    str(length: number) {
        let result = '';
        const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        const charactersLength = characters.length;
        let counter = 0;
        while (counter < length) {
            result += characters.charAt(Math.floor(Math.random() * charactersLength));
            counter += 1;
        }
        return result;
    },

    bytes(length: number) {
        return r(new Uint8Array(length))
    },

    u8() {
        return r(new Uint8Array(1))[0]
    },

    u16() {
        return r(new Uint16Array(1))[0]
    },

    u32() {
        return r(new Uint32Array(1))[0]
    },

    u64() {
        return r(new BigUint64Array(1))[0]
    },

    i8() {
        return r(new Int8Array(1))[0]
    },

    i16() {
        return r(new Int16Array(2))[0]
    },

    i32() {
        return r(new Int32Array(4))[0]
    },

    i64() {
        return r(new BigInt64Array(8))[0]
    },

    f32() {
        return rg(4).getFloat32(0)
    },

    f64() {
        return rg(8).getFloat64(0)
    }
}

function rg(len: number) {
    return new DataView(rand.bytes(len).buffer)
}

