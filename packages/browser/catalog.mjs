import { Plugin, PluginGroup, makePlugin, makeRule } from "./plugin.mjs";
import { data } from "./identity.mjs";
export function loadCatalog(runtime, catalog) {
  const groups = [];
  for (const definition of catalog.groups) {
    let group =
      definition.parent === null
        ? new PluginGroup()
        : groups[definition.parent].group();
    if (definition.name !== null) group = group.named(definition.name);
    groups.push(group);
  }
  const rules = catalog.rules.map((definition, index) =>
    makeRule(runtime, index, definition),
  );
  const plugins = catalog.plugins.map((definition) => {
    const plugin =
      definition.group === null ? new Plugin() : groups[definition.group].new();
    const state = data(plugin, "plugin");
    return makePlugin({
      ...state,
      name: definition.name,
      frozen: true,
      rules: definition.rules.map((index) => rules[index]),
    });
  });
  function tree(value, leaf) {
    if (typeof value === "number") return leaf(value);
    return Object.freeze(
      Object.fromEntries(
        Object.entries(value).map(([name, child]) => [name, tree(child, leaf)]),
      ),
    );
  }
  const presets = new Map();
  return {
    plugins: tree(catalog.exports.plugins, (index) => plugins[index]),
    presets: tree(catalog.exports.presets, (index) => {
      if (!presets.has(index)) {
        const definition = catalog.presets[index];
        presets.set(
          index,
          runtime.preset(
            {
              plugins: definition.plugins.map((index) =>
                data(plugins[index], "plugin"),
              ),
              order: definition.order,
            },
            definition,
          ),
        );
      }
      return presets.get(index);
    }),
  };
}
