import init, { fib } from "./pkg/deno_wasm_example.js";

await init();

console.log(fib(BigInt(10)));
