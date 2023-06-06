import * as use from './databuf.lib.ts'
let struct = {
}
let extern = {
}
export default class Self {
    constructor(private rpc: use.RpcTransport) { }
    static close(this: Self) { this.rpc.close() }
    add(_0: number, _1: number,) {
        const fn = this.rpc.unary()
        const d = new use.BufWriter(fn);
        d.u16(1);
        d.num('I', 32)(_0);
        d.num('I', 32)(_1);
        d.flush();
        return fn.call().then(buf => new use.Decoder(new Uint8Array(buf)))
            .then(d => d.num('I', 32)());
    }
    print(_0: string,) {
        const fn = this.rpc.unary()
        const d = new use.BufWriter(fn);
        d.u16(2);
        d.str(_0);
        d.flush();
    }
}
