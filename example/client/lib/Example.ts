import * as use from './databuf.lib.ts'
let struct = {
}
let extern = {
}
export default class Self {
    constructor(private rpc: use.RpcTransport) { }
    static close(this: Self) { this.rpc.close() }
    async get_data() {
        let fn = this.rpc.unary();
        let d = new use.BufWriter(fn);
        d.u16(1);
        d.flush();
        let _d = await fn.call();
        {
            let d = new use.Decoder(_d);
            return d.tuple(d.tuple(d.tuple(d.u8, d.num('U', 16), d.num('U', 32), d.num('U', 64), d.num('U', 128), d.num('U', 64),), d.tuple(d.u8, d.num('U', 16), d.num('U', 32), d.num('U', 64), d.num('U', 128), d.num('U', 64),), d.tuple(d.i8, d.num('I', 16), d.num('I', 32), d.num('I', 64), d.num('I', 128), d.num('I', 64),), d.tuple(d.i8, d.num('I', 16), d.num('I', 32), d.num('I', 64), d.num('I', 128), d.num('I', 64),), d.tuple(d.f32, d.f64,), d.tuple(d.f32, d.f64,),), d.tuple(d.bool, d.bool,), d.tuple(d.str, d.str, d.char,), d.tuple(d.option(d.str), d.option(d.str),), d.tuple(d.result(d.num('I', 32), d.str), d.result(d.num('I', 32), d.str),),)()
        }
    }
    async validate(_0: any) {
        let fn = this.rpc.unary();
        let d = new use.BufWriter(fn);
        d.u16(2);
        d.tuple(d.tuple(d.tuple(d.u8, d.num('U', 16), d.num('U', 32), d.num('U', 64), d.num('U', 128), d.num('U', 64),), d.tuple(d.u8, d.num('U', 16), d.num('U', 32), d.num('U', 64), d.num('U', 128), d.num('U', 64),), d.tuple(d.i8, d.num('I', 16), d.num('I', 32), d.num('I', 64), d.num('I', 128), d.num('I', 64),), d.tuple(d.i8, d.num('I', 16), d.num('I', 32), d.num('I', 64), d.num('I', 128), d.num('I', 64),), d.tuple(d.f32, d.f64,), d.tuple(d.f32, d.f64,),), d.tuple(d.bool, d.bool,), d.tuple(d.str, d.str, d.char,), d.tuple(d.option(d.str), d.option(d.str),), d.tuple(d.result(d.num('I', 32), d.str), d.result(d.num('I', 32), d.str),),)(_0);
        d.flush();
        let _d = await fn.call();
    }
}
