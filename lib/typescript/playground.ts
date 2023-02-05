// deno-lint-ignore-file
import * as use from "./mod.ts";

// const extern = {
//   BasicUser: (d: use.BufWriter, z: any) => {
//     d.str(z.name);
//     d.u8(z.age);
//     this.BasicCar.bind(null, d)(z.car);
//     this.BasicFoo.bind(null, d)(z.foo);
//   },
// }

const struct = {
  BasicUser(d: use.Decoder) {
    return {
      name: d.str(),
      age: d.u8(),
      car: this.BasicCar.bind(null, d)(),
      foo: this.BasicFoo.bind(null, d)(),
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
    const num = d.len_u15();
    switch (num) {
      case 0: return {
        type: "Quz" as const,
        x: d.u8(),
      };
      case 1: return { type: "Bar" as const, 0: d.u8(), 1: d.u16(), 2: this.BasicBez.bind(null, d)() };
      default: throw new Error('Unknown discriminant of `BasicFoo`: ' + num)
    }
  },
  BasicBez(d: use.Decoder) {
    return d.tuple(d.u8, d.u16);
  },
}

export enum BasicCar {
  Foo = 0,
  Bar = 1,
}