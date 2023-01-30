import { Decoder } from "./decode.ts";
type Decode<T> = (this: Decoder) => T;

export function Add(d: Decoder) {
  return {
    name: d.tuple(d.option(d.bool), d.result(d.str, d.str), d.map(d.str, d.set(d.u8)))(),
  };
}

// function tupels<T extends any[]>(...args: T) {
//   let n = "null" as { [K in keyof T]: ReturnType<T[K]> };
//   return n;
// }
