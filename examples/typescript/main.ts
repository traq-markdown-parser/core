import { readFile } from "node:fs/promises";
import { loadRuntime } from "@traptitech/markdown-parser";
import * as html from "@traptitech/markdown-renderer";
import * as commonmark from "@traptitech/markdown-renderer/common";
import MarkdownIt from "markdown-it";

const wasm = await readFile(
  new URL(import.meta.resolve("@traptitech/markdown-parser/parser.wasm")),
);
const runtime = await loadRuntime(new Uint8Array(wasm));
try {
  const parser = runtime.parser(runtime.presets.traq.v1);
  try {
    const document = parser.parse("**hello** :stamp: $x$");
    console.log(JSON.stringify(document, null, 2));

    // The adapter produces markdown-it tokens; parsing is handled by Wasm.
    // Extensions without a presentation handler remain escaped source text.
    const renderer = html.renderer(commonmark.preset());
    const markdown = new MarkdownIt();
    console.log(
      markdown.renderer.render(renderer.render(document), markdown.options, {}),
    );
  } finally {
    parser.dispose();
  }

  const grammar = runtime.presets.traq.v1
    .toBuilder()
    .remove(runtime.plugins.generic.math)
    .build();
  try {
    const withoutMath = runtime.parser(grammar);
    try {
      grammar.dispose(); // The Parser retains the compiled grammar.
      console.log(JSON.stringify(withoutMath.parseInline("$x$"), null, 2));
    } finally {
      withoutMath.dispose();
    }
  } finally {
    grammar.dispose(); // Repeated disposal is safe.
  }
} finally {
  runtime.dispose();
}
