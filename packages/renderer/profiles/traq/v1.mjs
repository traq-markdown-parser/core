import { generic } from "../../extensions/generic.mjs";
import { trap } from "../../extensions/trap.mjs";
import {
  createRenderer as createBase,
  installParser as installBase,
} from "../../index.mjs";
export const extensions = new Map([...generic, ...trap]);
const configured = (options) => ({
  ...options,
  extensions: new Map([...extensions, ...(options?.extensions ?? [])]),
});
export function createRenderer(options) {
  return createBase(configured(options));
}
export function installParser(renderer, parser, options) {
  return installBase(renderer, parser, configured(options));
}
