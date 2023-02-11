// deno-lint-ignore-file no-namespace
import { RPC } from "./transport.ts";
import * as use from "./databuf/mod.ts";

namespace struct {
  export function BasicUser(d: use.Decoder) {
    return {
      name: d.str(),
      age: d.u8(),
      car: BasicCar.bind(0, d)(),
      foo: BasicFoo.bind(0, d)(),
    }
  }
  export function BasicCar(d: use.Decoder) {
    const num = d.len_u15();
    switch (num) {
      case 0: return trait.BasicCar.Foo;
      case 1: return trait.BasicCar.Bar;

      default: throw new Error('Unknown discriminant of `BasicCar`: ' + num)
    }
  }
  export function BasicFoo(d: use.Decoder) {
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
        2: BasicBez.bind(0, d)(),
      };
        return x as typeof x;

      default: throw new Error('Unknown discriminant of `BasicFoo`: ' + num)
    }
  }
  export function BasicBez(d: use.Decoder) {
    return d.tuple(d.u8, d.u16,)();
  }
}

export namespace trait {
  export type BasicBez = ReturnType<typeof struct.BasicBez>;
  export enum BasicCar {
    Foo = 0,
    Bar = 1,
  }
  export type BasicFoo = ReturnType<typeof struct.BasicFoo>;
  export type BasicUser = ReturnType<typeof struct.BasicUser>;
}

namespace extern {
  export function BasicUser(d: use.BufWriter, z: trait.BasicUser) {
    d.str(z.name);
    d.u8(z.age);
    BasicCar.bind(0, d)(z.car);
    BasicFoo.bind(0, d)(z.foo);
  }
  export function BasicCar(d: use.BufWriter, z: trait.BasicCar) {
    switch (z) {
      case trait.BasicCar.Foo: return d.len_u15(0);
      case trait.BasicCar.Bar: return d.len_u15(1);
    }
  }
  export function BasicFoo(d: use.BufWriter, z: trait.BasicFoo) {
    switch (z.type) {
      case "Quz": d.len_u15(0);
        d.u8(z.x);
        break;
      case "Bar": d.len_u15(1);
        d.u8(z[0]);
        d.u16(z[1]);
        BasicBez.bind(0, d)(z[2]);
        break;
    }
  }
  export function BasicBez(d: use.BufWriter, z: trait.BasicBez) {
    return d.tuple(d.u8, d.u16,)(z);
  }
}

export default class mod {
  constructor(private rpc: RPC) { }
  static close(this: mod) { this.rpc.close() }

  user(_0: string, _1: number,): string {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(6);
    d.tuple(d.str, d.u8,);
    throw new Error('todo')
  }

  demo(): void {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(3)
    d.flush();
    throw new Error('todo')
  }

  get_user(_0: trait.BasicUser,) {
    const fn = this.rpc.unary_call()
    const d = new use.BufWriter(fn);
    d.u16(2)
    extern.BasicUser(d, _0);

    return fn.output().then(buf => new use.Decoder(new Uint8Array(buf)))
      // .then(d => d.str())
      .then(d => struct.BasicUser(d))
  }
}
