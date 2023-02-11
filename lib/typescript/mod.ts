export * from "./databuf/mod.ts"
import { Write } from "./databuf/mod.ts"

export interface RPC {
    unary_call(): Write & { output(): Promise<ArrayBuffer> }
    close(): void
}

export function enumErr(name: string, num: number) {
    return new Error(`Unknown discriminant of \`${name}\`: ${num}`)
}