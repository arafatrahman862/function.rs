// deno-lint-ignore-file
import * as use from "./mod.ts";

export interface BasicUser {
  name: string;
  age: number;
  car: BasicCar;
  foo: BasicFoo;
}

export enum BasicCar {
  Foo = 0,
  Bar = 1,
}

export type BasicFoo =
  | { type: "Quz"; x: number }
  | { type: "Bar"; 0: number; 1: number; 2: BasicBez };

export type BasicBez = [number, number];

const extern = {
  BasicUser(d: use.BufWriter, z: BasicUser) {
    d.str(z.name);
    d.u8(z.age);
    this.BasicCar.bind(this, d)(z.car);
    this.BasicFoo.bind(this, d)(z.foo);
  },
  BasicCar(d: use.BufWriter, z: BasicCar) {
    switch (z) {
      case BasicCar.Foo:
        return d.len_u15(0);
      case BasicCar.Bar:
        return d.len_u15(1);
    }
  },
  BasicFoo(d: use.BufWriter, z: BasicFoo) {
  },
  BasicBez(d: use.BufWriter, z: BasicBez) {
  },
};

const struct = {
  BasicUser(d: use.Decoder) {
    return {
      name: d.str(),
      age: d.u8(),
      car: this.BasicCar.bind(this, d)(),
      foo: this.BasicFoo.bind(this, d)(),
    };
  },
  BasicCar(d: use.Decoder) {
    const num = d.len_u15();
    switch (num) {
      case 0:
        return BasicCar.Foo;
      case 1:
        return BasicCar.Bar;

      default:
        throw new Error("Unknown discriminant of `BasicCar`: " + num);
    }
  },
  BasicFoo(d: use.Decoder) {
    let x;
    const num = d.len_u15();
    switch (num) {
      case 0:
        x = {
          type: "Quz" as const,
          x: d.u8(),
        };
        return x as typeof x;
      case 1:
        x = {
          type: "Bar" as const,
          0: d.u8(),
          1: d.u16(),
          2: this.BasicBez.bind(this, d)(),
        };
        return x as typeof x;
      case 6:
        x = {
          type: "Ba" as const,
        };
        return x as typeof x;

      default:
        throw new Error("Unknown discriminant of `BasicFoo`: " + num);
    }
  },
  BasicBez(d: use.Decoder) {
    return d.tuple(d.u8, d.u16)();
  },
};
