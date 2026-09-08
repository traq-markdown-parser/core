import type { Plugin } from "../index.mjs";
export interface Options {
  store?: unknown;
}
export function plugin(options?: Options): Plugin;
