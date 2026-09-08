function label(name) {
  if (typeof name !== "string") throw new TypeError("Expected display name");
  return name;
}

/** Identity is the instance; names are only used in diagnostics. */
export class PluginGroup {
  constructor(name, parent = null) {
    if (parent !== null && !(parent instanceof PluginGroup))
      throw new TypeError("Expected PluginGroup");
    this.name = label(name);
    this.parent = parent;
    Object.freeze(this);
  }
  group(name) {
    return new PluginGroup(name, this);
  }
  new(name) {
    return new Plugin(name, this);
  }
}

export class Plugin {
  constructor(name, namespace = null) {
    if (namespace !== null && !(namespace instanceof PluginGroup))
      throw new TypeError("Expected PluginGroup");
    this.name = label(name);
    this.namespace = namespace;
    Object.freeze(this);
  }
  static group(name) {
    return new PluginGroup(name);
  }
}
