import { data, owned } from "./identity.mjs";
import { GrammarBuildError } from "./errors.mjs";

export class GrammarBuilder {
  #runtime;
  #plugins;
  #order;
  constructor(runtime, snapshot = { plugins: [], order: [] }) {
    this.#runtime = runtime;
    this.#plugins = [...snapshot.plugins];
    this.#order = [...snapshot.order];
  }
  add(plugin) {
    const state = data(plugin, "plugin");
    if (this.#plugins.some((p) => p.symbol === state.symbol))
      throw new GrammarBuildError({
        code: "duplicate",
        element: state.name ?? "<anonymous plugin>",
      });
    for (const provider of state.text)
      if (provider.runtime !== this.#runtime) throw new TypeError("Text provider belongs to a different Runtime");
    const used = new Set(this.#order);
    for (const rule of state.rules) {
      const { index } = owned(rule, "rule", this.#runtime);
      if (used.has(index))
        throw new GrammarBuildError({
          code: "duplicate",
          element: rule.name ?? "<anonymous rule>",
        });
      used.add(index);
    }
    this.#plugins.push(state);
    this.#order.push(...state.rules.map((r) => data(r, "rule").index));
    return this;
  }
  remove(plugin) {
    const state = data(plugin, "plugin");
    const index = this.#plugins.findIndex((p) => p.symbol === state.symbol);
    if (index < 0)
      throw new GrammarBuildError({
        code: "missing",
        element: state.name ?? "<anonymous plugin>",
      });
    const removed = new Set(
      this.#plugins[index].rules.map((r) => data(r, "rule").index),
    );
    this.#plugins.splice(index, 1);
    this.#order = this.#order.filter((index) => !removed.has(index));
    return this;
  }
  before(rule, anchor) {
    const first = owned(rule, "rule", this.#runtime),
      next = owned(anchor, "rule", this.#runtime);
    if (first.phase !== next.phase)
      throw new TypeError("Rules must belong to the same phase");
    const from = this.#order.indexOf(first.index),
      to = this.#order.indexOf(next.index);
    if (from < 0 || to < 0)
      throw new GrammarBuildError({
        code: "missing",
        element: (from < 0 ? rule : anchor).name ?? "<anonymous rule>",
      });
    if (from !== to) {
      const [entry] = this.#order.splice(from, 1);
      this.#order.splice(from < to ? to - 1 : to, 0, entry);
    }
    return this;
  }
  build() {
    return this.#runtime.build({
      plugins: [...this.#plugins],
      order: [...this.#order],
    });
  }
}
export function composition(snapshot) {
  const groups = [],
    indices = new Map();
  function visit(group) {
    if (!group) return null;
    const state = data(group, "group");
    if (indices.has(state.symbol)) return indices.get(state.symbol);
    const parent = visit(state.parent),
      index = groups.length;
    groups.push({ parent, name: state.name });
    indices.set(state.symbol, index);
    return index;
  }
  const plugins = snapshot.plugins.map((p) => ({
    group: visit(p.group),
    name: p.name,
    rules: p.rules.map((rule) => data(rule, "rule").index),
    text: p.text.map((provider) => provider.index),
  }));
  return { groups, plugins, order: snapshot.order };
}
