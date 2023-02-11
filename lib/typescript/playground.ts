import { RPC } from "./transport.ts";
import * as use from "./databuf/mod.ts";

let struct = {
  BasicUser(d: use.Decoder) {
    return {
      name: d.str(),
      age: d.u8(),
      car: struct.BasicCar.bind(0, d)(),
      foo: struct.BasicFoo.bind(0, d)(),
    }
  },
  BasicCar(d: use.Decoder) {
    const num = d.len_u15();
    switch (num) {
      case 0: return BasicCar.Foo;
      case 1: return BasicCar.Bar;

      default: throw new Error('Unknown discriminant of `BasicCar`: ' + num)
    }
  },
  BasicFoo(d: use.Decoder) {
    let x;
    const num = d.len_u15();
    switch (num) {
      case 0: x = {
        type: "Quz" as const,
        x: d.u8(),
      };
        return x as typeof x;
      case 1: x = {
        type: "Bar" as const,
        0: d.u8(),
        1: struct.BasicBez.bind(0, d)(),
      };
        return x as typeof x;
      case 2: x = {
        type: "Many" as const,
        0: d.tuple(d.vec(struct.BasicFoo.bind(0, d)), d.vec(struct.BasicFoo.bind(0, d)),)(),
      };
        return x as typeof x;

      default: throw new Error('Unknown discriminant of `BasicFoo`: ' + num)
    }
  },
  BasicBez(d: use.Decoder) {
    return d.tuple(d.u8, d.u16,)();
  },
}
export type BasicBez = ReturnType<typeof struct.BasicBez>;
export enum BasicCar {
  Foo = 0,
  Bar = 1,
}
export type BasicFoo = ReturnType<typeof struct.BasicFoo>;
export type BasicUser = ReturnType<typeof struct.BasicUser>;

let extern = {
  BasicUser(d: use.BufWriter, z: BasicUser) {
    d.str(z.name);
    d.u8(z.age);
    extern.BasicCar.bind(0, d)(z.car);
    extern.BasicFoo.bind(0, d)(z.foo);
  },
  BasicCar(d: use.BufWriter, z: BasicCar) {
    switch (z) {
      case BasicCar.Foo: return d.len_u15(0);
      case BasicCar.Bar: return d.len_u15(1);
    }
  },
  BasicFoo(d: use.BufWriter, z: BasicFoo) {
    switch (z.type) {
      case "Quz": d.len_u15(0);
        d.u8(z.x);
        break;
      case "Bar": d.len_u15(1);
        d.u8(z[0]);
        extern.BasicBez.bind(0, d)(z[1]);
        break;
      case "Many": d.len_u15(2);
        d.tuple(d.vec(extern.BasicFoo.bind(0, d)), d.vec(extern.BasicFoo.bind(0, d)),)(z[0]);
        break;
    }
  },
  BasicBez(d: use.BufWriter, z: BasicBez) {
    return d.tuple(d.u8, d.u16,)(z);
  },
}
export default class mod {
  constructor(private rpc: RPC) { }
  static close(this: mod) { this.rpc.close() }
  user(_0: string, _1: number,) {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(6);
    d.str(_0);
    d.u8(_1);
    d.flush();
    return fn.output().then(buf => new use.Decoder(new Uint8Array(buf)))
      .then(d => d.str());
  }
  demo(_0: void,): void {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(3);
    (_0);
    d.flush();
  }
  get_user(_0: [number, BasicUser],) {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(2);
    d.tuple(d.u8, extern.BasicUser.bind(0, d),)(_0);
    d.flush();
    return fn.output().then(buf => new use.Decoder(new Uint8Array(buf)))
      .then(d => d.tuple(d.u8, struct.BasicUser.bind(0, d),)());
  }
}