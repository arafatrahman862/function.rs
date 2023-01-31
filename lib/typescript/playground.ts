import * as use from "./decode.ts";

export enum BasicCar {
  Foo,
  Bar,
}

const struct = {
  BasicUser: (d: use.Decoder) => ({
    name: d.str(),
    age: d.u8(),
    car: struct.BasicCar.bind(null, d)(),
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
};