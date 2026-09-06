import { metadata, data, label } from "./identity.mjs";

export class PluginGroup {
  constructor(parent = null) {
    metadata.set(this, { kind: "group", symbol: {}, parent, name: null });
  }
  named(name) {
    const group = new PluginGroup();
    metadata.set(group, { ...data(this, "group"), name: label(name) });
    return group;
  }
  get name() {
    return data(this, "group").name;
  }
  get parent() {
    return data(this, "group").parent;
  }
  group() {
    return new PluginGroup(this);
  }
  new() {
    return makePlugin({ ...data(new Plugin(), "plugin"), group: this });
  }
}

export class Plugin {
  constructor() {
    metadata.set(this, {
      kind: "plugin",
      symbol: {},
      name: null,
      group: null,
      rules: [],
      frozen: false,
    });
  }
  static group() {
    return new PluginGroup();
  }
  named(name) {
    return makePlugin({
      ...data(this, "plugin"),
      name: label(name),
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
  const plugin = new Plugin();
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
