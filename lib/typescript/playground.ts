import * as use from "./mod.ts";

export interface BasicUser {
  name: string,
  age: number,
  car: BasicCar,
  foo: BasicFoo,
}

export enum BasicCar {
  Foo = 0,
  Bar = 1,
}

export type BasicFoo =
  | { type: "Quz", x: number }
  | { type: "Bar", 0: number, 1: number }

const extern = {
  BasicUser: (e: use.BufWriter, z: BasicUser) => {
    e.str(z.name);
    e.u8(z.age);
    extern.BasicCar.bind(null, e)(z.car);
  },
  BasicCar: (e: use.BufWriter, z: BasicCar) => {

  }
}

const struct = {
  BasicUser: (d: use.Decoder) => ({
    name: d.str(),
    age: d.u8(),
    car: struct.BasicCar.bind(null, d)(),
    foo: struct.BasicFoo.bind(null, d)(),
  }),
  BasicCar: (d: use.Decoder) => {
    const num = d.len_u15();
    switch (num) {
      case 0: return BasicCar.Foo;
      case 1: return BasicCar.Bar;
      default: throw new Error('Unknown discriminant of `BasicCar`: ' + num)
    }
  },
  BasicFoo: (d: use.Decoder) => {
    const num = d.len_u15();
    switch (num) {
      case 0: return {
        type: "Quz" as const,
        x: d.u8(),
      };
      case 1: return { type: "Bar" as const, 0: d.u8(), 1: d.u16() };
      default: throw new Error('Unknown discriminant of `BasicFoo`: ' + num)
    }
  },
}