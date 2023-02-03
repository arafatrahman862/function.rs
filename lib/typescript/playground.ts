import * as use from "./decode.ts";

export const struct = {
  BasicUser: (d: use.Decoder) => ({
    name: d.str(),
    age: d.u8(),
    car: struct.BasicCar.bind(null, d)(),
    foo: struct.BasicFoo.bind(null, d)(),
  }),
  BasicCar: (d: use.Decoder) => {
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
  BasicFoo: (d: use.Decoder) => {
    const num = d.len_u15();
    switch (num) {
      case 0:
        return { type: "Quz" as const, x: d.u8() }
      case 1:
        return { type: "Bar", 0: d.u8(), 1: d.u16() } as const;
      default:
        throw new Error("Unknown discriminant of `BasicFoo`: " + num);
    }
  },
};

export enum BasicCar {
  Foo,
  Bar,
}