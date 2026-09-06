import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import {
  loadRuntime,
  Plugin,
  GrammarBuildError,
} from "../packages/browser/index.mjs";
const bytes = await readFile(
  new URL("../packages/browser/parser.wasm", import.meta.url),
);

test("shared namespaces, display names and adoption match core semantics", async (t) => {
  const r = await loadRuntime(bytes);
  t.after(() => r.dispose());
  const generic = Plugin.group().named("generic");
  const github = generic.group().named("github");
  const math = generic.new().named("math");
  const issue = github.new().named("math");
  const b = r.builder().add(math).add(issue);
  assert.throws(() => b.add(math.named("renamed")), GrammarBuildError);
  const g = b.build();
  t.after(() => g.dispose());
  assert.match(g.describe(), /generic\/github\/math/);
  const bad = r.builder().add(math).add(generic.new().named("math"));
  assert.throws(
    () => bad.build(),
    (e) => e.detail?.code === "duplicate_name",
  );
  const other = Plugin.group().named("generic");
  assert.throws(
    () => r.builder().add(math).add(other.new()).build(),
    GrammarBuildError,
  );
  const unnamed = r.builder().add(new Plugin()).add(new Plugin()).build();
  unnamed.dispose();
  b.remove(math.named("new display name"));
  b.build().dispose();
});

test("preset forks and parser leases remain independent", async (t) => {
  const r = await loadRuntime(bytes);
  t.after(() => r.dispose());
  const original = r.parser(r.presets.traq.v1);
  t.after(() => original.dispose());
  const b = r.presets.traq.v1.toBuilder().remove(r.plugins.generic.math);
  const g = b.build(),
    p = r.parser(g);
  assert.match(JSON.stringify(original.parse("$x$")), /generic\/math_inline/);
  assert.doesNotMatch(JSON.stringify(p.parse("$x$")), /generic\/math_inline/);
  g.dispose();
  g.dispose();
  assert.equal(p.parseInline("$x$").children[0].value, "$x$");
  assert.throws(() => r.parser(g), /disposed/);
  assert.equal(g.plugins.length, runtimePluginCount(r) - 1);
  assert.throws(
    () => g.plugins[0].add(r.plugins.trap.stamp.inlineRules[0]),
    /immutable/,
  );
  const rebuilt = g.toBuilder().build();
  rebuilt.dispose();
  p.dispose();
  p.dispose();
  assert.throws(() => p.parse("x"), /disposed/);
  // Registration snapshots definitions; edits never mutate a built grammar.
  const group = Plugin.group().named("custom"),
    plugin = group.new().named("math");
  const [rule] = r.plugins.generic.math.inlineRules;
  plugin.add(rule);
  const built = r.builder().add(r.plugins.commonmark.core).add(plugin).build();
  plugin.add(r.plugins.trap.stamp.inlineRules[0]);
  assert.throws(() => built.toBuilder().remove(plugin), GrammarBuildError);
  built.dispose();
});

test("bundled rules can be regrouped and reordered, foreign instances are rejected", async (t) => {
  const r = await loadRuntime(bytes),
    other = await loadRuntime(bytes);
  t.after(() => r.dispose());
  t.after(() => other.dispose());
  const stamp = r.plugins.trap.stamp,
    math = r.plugins.generic.math;
  const b = r.builder().add(r.plugins.commonmark.core).add(stamp).add(math);
  b.before(math.inlineRules[0], stamp.inlineRules[0]);
  const g = b.build(),
    p = r.parser(g);
  t.after(() => p.dispose());
  t.after(() => g.dispose());
  assert.match(JSON.stringify(p.parse(":stamp: $x$")), /trap\/stamp/);
  assert.throws(
    () => b.before(math.blockRules[0], stamp.inlineRules[0]),
    /phase/,
  );
  assert.throws(() => r.builder().add(other.plugins.generic.math), /Runtime/);
  assert.throws(() => r.parser(other.presets.traq.v1), /Runtime/);
  r.dispose();
  assert.throws(() => p.parse("x"), /disposed/);
});

function runtimePluginCount(runtime) {
  return runtime.presets.traq.v1.plugins.length;
}
