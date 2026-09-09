import test from "node:test";
import assert from "node:assert/strict";
import {
  renderer,
  Plugin,
  PresetBuilder,
} from "@traq-markdown-parser/core/renderer";
import { Plugin as Declaration } from "@traq-markdown-parser/core/definitions";
test("core renders custom AST nodes without a grammar or Wasm runtime", () => {
  const declaration = new Declaration("custom");
  const plugin = new Plugin(declaration).on("text", (n, c) => c.escape(n.data));
  const builder = new PresetBuilder().add(plugin);
  const view = renderer(builder.build());
  const document = {
    source: "<猫>",
    children: [{ kind: "text", data: "<猫>", span: { start: 0, end: 5 } }],
  };
  assert.equal(view.render(document), "&lt;猫&gt;");
  plugin.replace("text", () => "<b>changed</b>");
  assert.equal(view.render(document), "&lt;猫&gt;");
  assert.equal(
    renderer(new PresetBuilder().build()).render(document),
    "<p>&lt;猫&gt;</p>\n",
  );
  assert.throws(
    () => new PresetBuilder().add(plugin).add(plugin),
    /Duplicate plugin/,
  );
});
