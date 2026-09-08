import { Plugin as Declaration } from "@traptitech/markdown-definitions";
import { metadata, data } from "./identity.mjs";

export class Plugin {
  constructor(declaration) {
    if (!(declaration instanceof Declaration))
      throw new TypeError("Expected Plugin declaration");
    metadata.set(this, {
      kind: "plugin",
      symbol: {},
      declaration,
      name: declaration.name,
      group: declaration.namespace,
      rules: [],
      text: [],
      frozen: false,
    });
  }

  get name() {
    return data(this, "plugin").name;
  }
  get namespace() {
    return data(this, "plugin").group;
  }
  get inlineRules() {
    return Object.freeze(
      data(this, "plugin").rules.filter((r) => r.phase === "inline"),
    );
  }
  get blockRules() {
    return Object.freeze(
      data(this, "plugin").rules.filter((r) => r.phase === "block"),
    );
  }
  get textRules() {
    return Object.freeze(
      data(this, "plugin").rules.filter((r) => r.phase === "text"),
    );
  }
  add(rule) {
    data(rule, "rule");
    const previous = data(this, "plugin");
    if (previous.frozen)
      throw new TypeError("Bundled plugin definitions are immutable");
    metadata.set(this, {
      ...previous,
      symbol: {},
      rules: [...previous.rules, rule],
    });
    return this;
  }
}
export function makePlugin(state) {
  const plugin = new Plugin(state.declaration);
  metadata.set(plugin, state);
  return plugin;
}
export function makeRule(runtime, index, definition) {
  const rule = Object.freeze({
    name: definition.name,
    phase: definition.phase,
  });
  metadata.set(rule, { kind: "rule", runtime, index, ...definition });
  return rule;
}
