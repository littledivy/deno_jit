import { Plug } from "https://deno.land/x/plug/mod.ts";
const path = "target/debug";
const options: Plug.Options = {
  name: "deno_jit",
  urls: {
    darwin: `${path}/libdeno_jit.dylib`,
    windows: `${path}/deno_jit.dll`,
    linux: `${path}/libdeno_jit.so`,
  },
  cache: ".",
};

const rid = await Plug.prepare(options);

const { jit_compile } = Plug.core.ops();

export function jit(inst: Uint8Array) {
  return (...args: any[]) => {
    return Plug.core.dispatch(
      jit_compile,
      inst,
      new Uint8Array([...args]),
    )![0];
  };
}
