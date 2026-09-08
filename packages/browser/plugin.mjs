import { metadata, data, label } from "./identity.mjs";

export class PluginGroup {
  constructor(name, parent = null) {
    metadata.set(this, { kind: "group", symbol: {}, parent, name: label(name) });
  }

  get name() {
    return data(this, "group").name;
  }
  get parent() {
    return data(this, "group").parent;
  }
  group(name) {
    return new PluginGroup(name, this);
  }
  new(name) {
    return makePlugin({ ...data(new Plugin(name), "plugin"), group: this });
  }
}

export class Plugin {
  constructor(name) {
    metadata.set(this, {
      kind: "plugin",
      symbol: {},
      name: label(name),
      group: null,
      rules: [],
      text: [],
      frozen: false,
    });
  }
  static group(name) {
    return new PluginGroup(name);
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
  const plugin = new Plugin(state.name);
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
