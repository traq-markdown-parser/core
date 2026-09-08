import { names } from "@traptitech/markdown-parser/nodes";
import test from "node:test";
import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import { loadRuntime } from "../packages/browser/index.mjs";
import { createRenderer } from "../packages/renderer/index.mjs";
import { createRenderer as createTraqRenderer } from "../packages/renderer/profiles/traq/v1.mjs";
const runtime = await loadRuntime(
  await readFile(new URL("../packages/browser/parser.wasm", import.meta.url)),
);

const parser = runtime.parser(runtime.presets.traq.v1);
test("one override retains all other common handlers and does not mutate defaults", () => {
  const document = parser.parse("**bold** [link](https://example.com)");
  const custom = createRenderer({
    overrides: { [names.Link]: (node, ctx) => ctx.inline(node.children) },
  });
  const ordinary = createRenderer();
  const types = (render) =>
    render
      .render(document)
      .flatMap((n) => [n.type, ...(n.children ?? []).map((n) => n.type)]);
  assert(types(custom).includes("strong_open"));
  assert(!types(custom).includes("link_open"));
  assert(types(ordinary).includes("link_open"));
  assert.throws(
    () => createRenderer({ overrides: { typo: () => [] } }),
    /Unknown common node/,
  );
});
test("profile extension overrides preserve other defaults and can disable one handler", () => {
  const document = parser.parse(":stamp: ==marked==");
  const custom = createTraqRenderer({
    extensions: new Map([[names.Stamp, null]]),
  });
  const children = custom.render(document)[1].children;
  assert(children.some((n) => n.type === "text" && n.content === ":stamp:"));
  assert(children.some((n) => n.type === "mark_open"));
  assert(
    createTraqRenderer()
      .render(document)[1]
      .children.some((n) => n.type === "regexp-0"),
  );
});
test("standalone defaults are usable without application state and escape unknown nodes", () => {
  const source = '!{"type":"user","id":"u","raw":"@user"}';
  assert.equal(
    createTraqRenderer().render(parser.parse(source))[1].children[0].content,
    "@user",
  );
  const unsafe = {
    source: "x",
    children: [
      {
        kind: names.Link,
        span: { start: 0, end: 1 },
        data: { destination: "javascript:alert(1)",
        title: null,
        form: "explicit" },
        children: [{ kind: names.Text, span: { start: 0, end: 1 }, data: {value: "x"} }],
      },
    ],
  };
  assert(
    !createRenderer()
      .renderInline(unsafe)[0]
      .children.some((n) => n.type === "link_open"),
  );
});
