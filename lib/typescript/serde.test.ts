import { rand } from "./fuzz.ts";
import { BufWriter } from "./encode.ts";
import { Decoder } from "./decode.ts";
import { assertEquals } from "https://deno.land/std@0.175.0/testing/asserts.ts";
import { Option, Result, WriteSync } from "./transport.ts";

class DefaultWriter implements WriteSync {
    bytes: number[] = []
    write(bytes: Uint8Array): number {
        this.bytes = this.bytes.concat(...bytes)
        return bytes.byteLength
    }
    flush() { }
}

console.clear()

Deno.test("Serde test: number", () => {
    const u8 = rand.u8();
    const u16 = rand.u16();
    const u32 = rand.u32();
    const u64 = rand.u64();

    const i8 = rand.i8();
    const i16 = rand.i16();
    const i32 = rand.i32();
    const i64 = rand.i64();

    const f32 = rand.f32();
    const f64 = rand.f64();

    // -----------------------------------

    const writer = new DefaultWriter();
    const encoder = new BufWriter(writer);

    encoder.u8(u8)
    encoder.u16(u16)
    encoder.u32(u32)
    encoder.u64(u64)

    encoder.i8(i8)
    encoder.i16(i16)
    encoder.i32(i32)
    encoder.i64(i64)

    encoder.f32(f32)
    encoder.f64(f64)

    encoder.flush()
    assertEquals(writer.bytes.length, 42)

    const decoder = new Decoder(new Uint8Array(writer.bytes));

    assertEquals(decoder.u8(), u8)
    assertEquals(decoder.u16(), u16)
    assertEquals(decoder.u32(), u32)
    assertEquals(decoder.u64(), u64)

    assertEquals(decoder.i8(), i8)
    assertEquals(decoder.i16(), i16)
    assertEquals(decoder.i32(), i32)
    assertEquals(decoder.i64(), i64)

    assertEquals(decoder.f32(), f32)
    assertEquals(decoder.f64(), f64)

    assertEquals(decoder.offset, 42)
})

Deno.test("Serde test: string, bytes", () => {
    const str = rand.str(rand.u8());
    const bytes = [...rand.bytes(rand.u8())];

    const writer = new DefaultWriter();
    const encoder = new BufWriter(writer);

    encoder.str(str)
    encoder.vec(encoder.u8)(bytes)

    encoder.flush()
    const decoder = new Decoder(new Uint8Array(writer.bytes));

    assertEquals(decoder.str(), str)
    assertEquals(decoder.set(decoder.u8)(), bytes)
})

Deno.test("Serde test: common type", () => {
    const char = '4';
    const bool = true;

    const some = "some";
    const none = null;

    type Vec2d = [number, number];
    const ok: Result<Vec2d, string> = { type: "Ok", value: [4, 2] };
    const err: Result<string, Vec2d> = { type: "Err", value: [2, 4] };

    const writer = new DefaultWriter();
    const encoder = new BufWriter(writer);

    encoder.char(char)
    encoder.bool(bool)

    encoder.option(encoder.str)(some)
    encoder.option(encoder.str)(none)

    encoder.result(encoder.arr(encoder.u8), encoder.str)(ok);
    encoder.result(encoder.str, encoder.arr(encoder.u8))(err);

    encoder.flush()
    const decoder = new Decoder(new Uint8Array(writer.bytes));

    assertEquals(decoder.char(), char)
    assertEquals(decoder.bool(), bool)

    assertEquals(decoder.option(decoder.str)(), some)
    assertEquals(decoder.option(decoder.str)(), none)

    assertEquals(decoder.result(decoder.arr(decoder.u8, 2), decoder.str)(), ok);
    assertEquals(decoder.result(decoder.str, decoder.arr(decoder.u8, 2))(), err);
})

Deno.test("Serde test: Complex type", () => {
    const value = new Map();
    value.set(0, null)
    value.set(1, "some")

    type ResultTy = Result<Map<number, Option<string>>, Array<string>>;
    const ok: ResultTy = { type: "Ok", value };
    const err: ResultTy = {
        type: "Err",
        value: ["Error: 1", "Error: 2"]
    };

    const writer = new DefaultWriter();
    const e = new BufWriter(writer);

    const encode = e.result(e.map(e.u8, e.option(e.str)), e.vec(e.str))
    encode(ok)
    encode(err)

    e.flush()
    const d = new Decoder(new Uint8Array(writer.bytes));

    const decode = d.result(d.map(d.u8, d.option(d.str)), d.set(d.str));
    assertEquals(decode(), ok);
    assertEquals(decode(), err);
})
