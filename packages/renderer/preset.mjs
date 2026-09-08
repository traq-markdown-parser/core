import { implementation } from "./plugin.mjs";

const presets = new WeakMap();
export function handlers(preset) {
  const value = presets.get(preset);
  if (!value) throw new TypeError("Expected renderer Preset");
  return value;
}

// Compare names only within selected declarations and their ancestor scopes.
function validateNames(plugins) {
  const scopes = new Map(),
    groups = new Set();
  function add(parent, name) {
    let names = scopes.get(parent);
    if (!names) scopes.set(parent, (names = new Set()));
    if (names.has(name)) throw new Error("Duplicate name: " + name);
    names.add(name);
  }
  function group(value) {
    if (!value || groups.has(value)) return;
    group(value.parent);
    add(value.parent, value.name);
    groups.add(value);
  }
  for (const { declaration } of plugins) group(declaration.namespace);
  for (const { declaration } of plugins)
    add(declaration.namespace, declaration.name);
}

export class PresetBuilder {
  #plugins = [];
  add(plugin) {
    const state = implementation(plugin);
    for (const existing of this.#plugins) {
      if (existing === state) throw new Error("Duplicate plugin");
      for (const kind of state.handlers.keys())
        if (existing.handlers.has(kind))
          throw new Error("Duplicate handler: " + kind);
    }
    this.#plugins.push(state);
    return this;
  }
  remove(plugin) {
    const index = this.#plugins.indexOf(implementation(plugin));
    if (index < 0) throw new Error("Missing plugin");
    this.#plugins.splice(index, 1);
    return this;
  }
  build() {
    validateNames(this.#plugins);
    const preset = Object.freeze({});
    presets.set(preset, new Map(this.#plugins.flatMap((p) => [...p.handlers])));
    return preset;
  }
}
