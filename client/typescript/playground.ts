import * as use from "./mod.ts";

export type BasicBez = [number, number];
export enum BasicCar {
  Foo = 0,
  Bar = 1,
}
export type BasicFoo =
  | { type: "Quz", x: number }
  | { type: "Bar", 0: number, 1: BasicBez }
  | { type: "Many", 0: [Array<BasicFoo>, Array<BasicFoo>] }
export interface BasicUser {
  name: string,
  age: number,
  car: BasicCar,
  foo: BasicFoo,
}
let struct = {
  BasicUser(d: use.Decoder): BasicUser {
    return {
      name: d.str(),
      age: d.u8(),
      car: struct.BasicCar.bind(0, d)(),
      foo: struct.BasicFoo.bind(0, d)(),
    }
  },
  BasicCar(d: use.Decoder): BasicCar {
    const num = d.len_u15();
    switch (num) {
      case 0: return BasicCar.Foo;
      case 1: return BasicCar.Bar;
      default: throw use.enumErr("BasicCar", num);
    }
  },
  BasicFoo(d: use.Decoder): BasicFoo {
    const num = d.len_u15();
    switch (num) {
      case 0: return {
        type: "Quz",
        x: d.u8(),
      };
      case 1: return {
        type: "Bar",
        0: d.u8(),
        1: struct.BasicBez.bind(0, d)(),
      };
      case 2: return {
        type: "Many",
        0: d.tuple(d.vec(struct.BasicFoo.bind(0, d)), d.vec(struct.BasicFoo.bind(0, d)),)(),
      };
      default: throw use.enumErr("BasicFoo", num);
    }
  },
  BasicBez(d: use.Decoder): BasicBez {
    return d.tuple(d.u8, d.u16,)();
  },
}
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
  constructor(private rpc: use.RPC) { }
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
  demo(_0: void,) {
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