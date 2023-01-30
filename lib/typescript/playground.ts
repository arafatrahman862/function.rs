import * as use from "./decode.ts";

// const struct = {
//   Add(d: use.Decoder) {
//     return {
//       /** awdawd */
//       name: d.tuple(
//         d.option(d.bool),
//         d.result(d.str, d.str),
//         d.map(d.str, d.set(d.u8)),
//       )(),
//     };
//   },
// };
// /** Hello */
// interface Add extends ReturnType<typeof struct.Add> {}
// // export type Add = ReturnType<typeof struct.Add>;

// function ad(): Add {
//   let d = 5 as unknown as use.Decoder;
//   return struct.Add(d)
// }

// let fg = ad()

// fg.name;

// const struct = {
//   BasicUser: (d: use.Decoder) => ({
//     name: d.str(),
//     age: d.u8(),
//     car: BasicCar(d),
//   }),
// };
