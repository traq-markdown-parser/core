import { names } from "@traptitech/markdown-parser/nodes";
import { Plugin as Declaration } from "@traptitech/markdown-definitions";
import test from "node:test";
import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import MarkdownIt from "markdown-it";
import { loadRuntime, Plugin as Syntax } from "../packages/browser/index.mjs";
import * as html from "../packages/renderer/index.mjs";
import * as common from "../packages/renderer/common.mjs";
import { plugin as generic } from "../packages/renderer/extensions/generic.mjs";
import { plugin as trap } from "../packages/renderer/extensions/trap.mjs";
import * as traq from "../packages/renderer/profiles/traq/index.mjs";
const bytes = await readFile(
  new URL("../packages/browser/parser.wasm", import.meta.url),
);
const runtime = await loadRuntime(bytes);
const parser = runtime.parser(runtime.presets.traq.v1);
test.after(() => {
  parser.dispose();
  runtime.dispose();
});
const build = (plugin) => new html.PresetBuilder().add(plugin).build();
const types = (render, document) =>
  render
    .render(document)
    .flatMap((n) => [n.type, ...(n.children ?? []).map((n) => n.type)]);

test("replacement preserves defaults and earlier snapshots", () => {
  const document = parser.parse("**bold** [link](https://example.com)");
  const plugin = common.html.plugin();
  const builder = new html.PresetBuilder().add(plugin);
  const before = html.renderer(builder.build());
  plugin.replace(common.nodes.Link, (node, ctx) => ctx.inline(node.children));
  assert.throws(() => builder.remove(plugin), /Missing plugin/);
  const custom = html.renderer(build(plugin));
  assert(types(custom, document).includes("strong_open"));
  assert(!types(custom, document).includes("link_open"));
  assert(types(before, document).includes("link_open"));
  assert(types(html.renderer(builder.build()), document).includes("link_open"));
  assert.throws(() => plugin.replace("typo", () => []), /Missing handler/);
  assert.throws(() => plugin.on(names.Link, () => []), /Duplicate handler/);
});

test("one declaration can supply independent syntax and rendering implementations", () => {
  const group = Declaration.group("custom"),
    declaration = group.new("math");
  const syntax = new Syntax(declaration).add(
    runtime.plugins.generic.math.inlineRules[0],
  );
  const presentation = new html.Plugin(declaration).on(
    names.InlineMath,
    (node, ctx) => ctx.fallback(node),
  );
  assert.equal(syntax.namespace, group);
  const grammar = runtime
    .builder()
    .add(runtime.plugins.commonmark.core)
    .add(syntax)
    .build();
  const local = runtime.parser(grammar);
  try {
    const rendered = html.renderer(
      new html.PresetBuilder().add(common.plugin()).add(presentation).build(),
    );
    assert.equal(
      rendered.render(local.parse("$x$"))[1].children[0].content,
      "$x$",
    );
  } finally {
    local.dispose();
    grammar.dispose();
  }
});

test("composition validates selected names and handlers without changing earlier presets", () => {
  const group = Declaration.group("custom");
  const first = new html.Plugin(group.new("one")).on("a", () => []);
  const builder = new html.PresetBuilder().add(first);
  const preset = builder.build();
  assert.throws(() => builder.add(first), /Duplicate plugin/);
  assert.throws(
    () => builder.add(new html.Plugin(group.new("two")).on("a", () => [])),
    /Duplicate handler/,
  );
  assert.doesNotThrow(() => builder.build());
  builder.remove(first);
  assert.doesNotThrow(() => html.renderer(preset));
  assert.throws(() => builder.remove(first), /Missing plugin/);
  builder.add(first).add(new html.Plugin(group.new("one")));
  assert.throws(() => builder.build(), /Duplicate name/);
  const other = Declaration.group("custom");
  assert.throws(
    () =>
      new html.PresetBuilder()
        .add(first)
        .add(new html.Plugin(other.new("different")))
        .build(),
    /Duplicate name/,
  );
  assert.throws(() => html.renderer({}), /Expected renderer Preset/);
  assert.throws(() => new html.Plugin("name"), /declaration/);
});

test("fallback replacement preserves other extensions and has no store requirement", () => {
  const extension = trap();
  extension.replace(names.Stamp, (node, ctx) => ctx.fallback(node));
  const custom = html.renderer(
    new html.PresetBuilder()
      .add(common.plugin())
      .add(generic())
      .add(extension)
      .build(),
  );
  const document = parser.parse(":stamp: ==marked==");
  const children = custom.render(document)[1].children;
  assert(children.some((n) => n.type === "text" && n.content === ":stamp:"));
  assert(children.some((n) => n.type === "mark_open"));
  assert(types(html.renderer(traq.v1.html()), document).includes("regexp-0"));
  const source = '!{"type":"user","id":"u","raw":"@user"}';
  assert.equal(
    html.renderer(traq.v1.html()).render(parser.parse(source))[1].children[0]
      .content,
    "@user",
  );
});

test("empty presets escape source and never implicitly enable CommonMark", () => {
  const render = html.renderer(new html.PresetBuilder().build());
  const document = parser.parse("**bold**");
  assert(!types(render, document).includes("strong_open"));
  assert.equal(render.render(document)[1].children[0].content, "**bold**");
  const unknown = {
    source: "<script>日本語</script>",
    children: [{ kind: "unknown", data: {}, span: { start: 0, end: 28 } }],
  };
  const md = new MarkdownIt();
  assert(
    !md.renderer
      .render(render.render(unknown), md.options, {})
      .includes("<script>"),
  );
});

test("tight lists do not hide paragraphs in blockquotes or loose nested lists", () => {
  const md = new MarkdownIt({ html: false });
  const cmParser = runtime.parser(runtime.presets.commonmark);
  try {
    const render = html.renderer(common.preset());
    for (const source of [
      "- one\n- two",
      "- one\n\n- two",
      "- outer\n  - inner\n\n  - loose",
      "- outer\n  > quote",
      "1. parent\n   - child\n     > quote",
      "- **strong**\n\n  paragraph",
    ]) {
      assert.equal(
        md.renderer.render(
          render.render(cmParser.parse(source)),
          md.options,
          {},
        ),
        md.render(source),
        source,
      );
    }
  } finally {
    cmParser.dispose();
  }
});

test("adapter uses supplied parser and renderer, and CommonMark owns link policy", () => {
  const md = new MarkdownIt();
  const render = html.renderer(common.preset({ validateLink: () => false }));
  const target = { md };
  assert.equal(html.installParser(target, parser, render), target);
  assert(!md.render("[link](https://example.com)").includes("href="));
  assert(!md.renderInline("[link](https://example.com)").includes("href="));
});

test("default link policy rejects unsafe destinations without losing their labels", () => {
  const document = parser.parseInline("[label](https://example.com)");
  document.children[0].data.destination = "javascript:alert(1)";
  const render = html.renderer(common.preset());
  const tokens = render.renderInline(document)[0].children;
  assert(!tokens.some((token) => token.type === "link_open"));
  assert.equal(tokens[0].content, "label");
});
