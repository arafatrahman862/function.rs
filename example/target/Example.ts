import { use } from './databuf.lib'
let struct = {
}
let extern = {
}
export default class Self {
    constructor(private rpc: use.RPC) { }
    static close(this: Self) { this.rpc.close() }
    add(_0: number, _1: number,) {
        const fn = this.rpc.unary_call(), d = new use.BufWriter(fn);
        // d.u16(1);
        d.num("I", 32)(_0);
        d.num("I", 32)(_1);
        d.flush();
        return fn.output().then(buf => new use.Decoder(new Uint8Array(buf)))
            .then(d => d.i32());
    }
    print(_0: string,) {
        const fn = this.rpc.unary_call(), d = new use.BufWriter(fn);
        // d.u16(5);
        d.str(_0);
        d.flush();
    }
}
