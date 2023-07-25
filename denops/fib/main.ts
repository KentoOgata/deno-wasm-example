import { Denops } from "https://deno.land/x/denops_core@v5.0.0/mod.ts";
import { is } from "https://deno.land/x/unknownutil@v3.2.0/mod.ts";
import init, { fib } from "../../pkg/deno_wasm_example.js";

export async function main(denops: Denops) {
  await init();

  denops.dispatcher = {
    // deno-lint-ignore ban-types
    fib: (n: unknown): BigInt => {
      if (is.Number(n)) {
        return fib(BigInt(n));
      } else if (is.String(n)) {
        const parsed = Number.parseInt(n);
        if (!isNaN(parsed)) {
          return fib(BigInt(parsed));
        }
      }

      return BigInt(0);
    },
  };

  await denops.cmd(
    `command -nargs=1 EchoFib echo denops#request('${denops.name}', 'fib', [<f-args>])`,
  );
}
