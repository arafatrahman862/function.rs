import { RPC } from "./transport.ts";
import * as use from "./databuf/mod.ts";

const struct = {
  BasicUser(d: use.Decoder) {
    return {
      name: d.str(),
      age: d.u8(),
      car: this.BasicCar.bind(this, d)(),
      foo: this.BasicFoo.bind(this, d)(),
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
        1: d.u16(),
        2: this.BasicBez.bind(this, d)(),
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

const extern = {
  BasicUser(d: use.BufWriter, z: BasicUser) {
    d.str(z.name);
    d.u8(z.age);
    this.BasicCar.bind(this, d)(z.car);
    this.BasicFoo.bind(this, d)(z.foo);
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
        d.u16(z[1]);
        this.BasicBez.bind(this, d)(z[2]);
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
  async user(_0: string, _1: number,) {
    const fn = this.rpc.unary_call();
    const d = new use.BufWriter(fn);
    d.u16(6);
    d.tuple(d.str, d.u16)([_0, _1]);
    d.flush();

    return new use.Decoder(new Uint8Array(await fn.output())).str()
  }
  get_user(_0: BasicUser,): BasicUser {
    const fn = this.rpc.unary_call();
    const d = new use.BufWriter(fn);
    d.u16(2);
    extern.BasicUser(d, _0);
    // d.tuple(extern.BasicUser)([_0]);
    throw "todo"
  }
}