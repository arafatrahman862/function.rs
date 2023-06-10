#!/usr/bin/env -S deno run --allow-net=localhost --unsafely-ignore-certificate-errors=localhost

import { assertEquals, assertRejects } from "https://deno.land/std@0.175.0/testing/asserts.ts";
import { HttpTransport } from "../../lib/frpc-codegen/client/typescript/http.transport.ts";
import Lib, { Echo_Log as Log } from "../../target/rpc/EchoTest.ts";

let lib = new Lib(new HttpTransport("https://localhost:4433/rpc/echo"));

let MAX_U8 = (1 << 8) - 1
let MAX_U16 = (1 << 16) - 1
let MAX_U32 = Number((1n << 32n) - 1n)
let MAX_U64 = (1n << 64n) - 1n
let MAX_U128 = (1n << 128n) - 1n

let MIN_I8 = -(1 << 8 - 1);
let MIN_I16 = -(1 << 16 - 1);
let MIN_I32 = Number(-(1n << 32n - 1n))
let MIN_I64 = -(1n << 64n - 1n)
let MIN_I128 = -(1n << 128n - 1n)

let MAX_I8 = -MIN_I8 - 1;
let MAX_I16 = -MIN_I16 - 1;
let MAX_I32 = -MIN_I32 - 1
let MAX_I64 = -MIN_I64 - 1n
let MAX_I128 = -MIN_I128 - 1n

await lib.log(Log.Disable);

// -------------------------------------------------------

assertEquals(0, await lib.echo_u8(0));
assertEquals(0, await lib.echo_u16(0));
assertEquals(0, await lib.echo_u32(0));
assertEquals(0n, await lib.echo_u64(0n));
assertEquals(0n, await lib.echo_u128(0n));

assertEquals(MAX_U8, await lib.echo_u8(MAX_U8));
assertEquals(MAX_U16, await lib.echo_u16(MAX_U16));
assertEquals(MAX_U32, await lib.echo_u32(MAX_U32));
assertEquals(MAX_U64, await lib.echo_u64(MAX_U64));
assertEquals(MAX_U128, await lib.echo_u128(MAX_U128));

assertRejects(() => lib.echo_u8(-1));
assertRejects(() => lib.echo_u16(-1));
assertRejects(() => lib.echo_u32(-1));
assertRejects(() => lib.echo_u64(-1n));
assertRejects(() => lib.echo_u128(-1n));

assertRejects(() => lib.echo_u8(MAX_U8 + 1));
assertRejects(() => lib.echo_u16(MAX_U16 + 1));
assertRejects(() => lib.echo_u32(MAX_U32 + 1));
assertRejects(() => lib.echo_u64(MAX_U64 + 1n));
assertRejects(() => lib.echo_u128(MAX_U128 + 1n));

// -------------------------------------------------------

assertEquals(MIN_I8, await lib.echo_i8(MIN_I8));
assertEquals(MIN_I16, await lib.echo_i16(MIN_I16));
assertEquals(MIN_I32, await lib.echo_i32(MIN_I32));
assertEquals(MIN_I64, await lib.echo_i64(MIN_I64));
assertEquals(MIN_I128, await lib.echo_i128(MIN_I128));

assertEquals(MAX_I8, await lib.echo_i8(MAX_I8));
assertEquals(MAX_I16, await lib.echo_i16(MAX_I16));
assertEquals(MAX_I32, await lib.echo_i32(MAX_I32));
assertEquals(MAX_I64, await lib.echo_i64(MAX_I64));
assertEquals(MAX_I128, await lib.echo_i128(MAX_I128));

assertRejects(() => lib.echo_i8(MIN_I8 - 1));
assertRejects(() => lib.echo_i16(MIN_I16 - 1));
assertRejects(() => lib.echo_i32(MIN_I32 - 1));
assertRejects(() => lib.echo_i64(MIN_I64 - 1n));
assertRejects(() => lib.echo_i128(MIN_I128 - 1n));

assertRejects(() => lib.echo_i8(MAX_I8 + 1));
assertRejects(() => lib.echo_i16(MAX_I16 + 1));
assertRejects(() => lib.echo_i32(MAX_I32 + 1));
assertRejects(() => lib.echo_i64(MAX_I64 + 1n));
assertRejects(() => lib.echo_i128(MAX_I128 + 1n));

// -------------------------------------------------------

