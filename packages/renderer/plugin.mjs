import { Plugin as Declaration } from "@traptitech/markdown-definitions";

const implementations = new WeakMap();
export function implementation(plugin) {
  const value = implementations.get(plugin);
  if (!value) throw new TypeError("Expected renderer Plugin");
  return value;
}

export class Plugin {
  constructor(declaration) {
    if (!(declaration instanceof Declaration))
      throw new TypeError("Expected Plugin declaration");
    implementations.set(this, { declaration, handlers: new Map() });
  }
  on(kind, handler) {
    return this.#set(kind, handler, false);
  }
  replace(kind, handler) {
    return this.#set(kind, handler, true);
  }
  #set(kind, handler, replacing) {
    if (typeof kind !== "string" || typeof handler !== "function")
      throw new TypeError("Expected node kind and render handler");
    const state = implementation(this);
    if (state.handlers.has(kind) !== replacing)
      throw new Error(
        (replacing ? "Missing" : "Duplicate") + " handler: " + kind,
      );
    const handlers = new Map(state.handlers);
    handlers.set(kind, handler);
    implementations.set(this, { declaration: state.declaration, handlers });
    return this;
  }
}
