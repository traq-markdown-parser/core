import { names } from "@traptitech/markdown-parser/nodes";
import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { loadRuntime } from "@traptitech/markdown-parser";
import * as html from "@traptitech/markdown-renderer";
import * as common from "@traptitech/markdown-renderer/common";

const bytes = await readFile(
  new URL(import.meta.resolve("@traptitech/markdown-parser/parser.wasm")),
);
assert.equal(createHash("sha256").update(bytes).digest("hex"), process.argv[2]);
const runtime = await loadRuntime(bytes);
try {
  const parser = runtime.parser(runtime.presets.traq.v1);
  try {
    assert.equal(
      parser.parse("**package**").children[0].children[0].kind,
      names.Strong,
    );
    assert.equal(
      parser.parse(":stamp:").children[0].children[0].kind,
      names.Stamp,
    );
    assert.equal(
      parser.parse("- parent\n\t- child").children[0].children[0].children[1]
        .kind,
      names.List,
    );
    const render = html.renderer(common.preset());
    assert.equal(
      render.render(parser.parse(":stamp:"))[1].children[0].content,
      ":stamp:",
    );
  } finally {
    parser.dispose();
  }
} finally {
  runtime.dispose();
}
console.log("Packed artifact, runtime, and NodeNext types passed");
