import { names } from "@traptitech/markdown-parser/nodes";
import {
  MarkdownParseError,
  type ReferenceData,
} from "@traptitech/markdown-parser";
import { loadRuntime, Plugin, isKnownNode, type Node, type ParserOptions } from "@traptitech/markdown-parser";
import { createRenderer } from "@traptitech/markdown-renderer";
import { extensions } from "@traptitech/markdown-renderer/traq/v1";
const runtime = await loadRuntime(new Uint8Array());
const grammar = runtime.presets.traq.v1
  .toBuilder()
  .remove(runtime.plugins.generic.math)
  .build();
const core = runtime.parser(grammar);
const generic = Plugin.group("custom");
const plugin = generic
  .new("math")
  .add(runtime.plugins.generic.math.inlineRules[0]);
runtime
  .builder()
  .add(runtime.plugins.commonmark.core)
  .add(plugin)
  .build()
  .dispose();
const math = runtime.plugins.generic.math;
// @ts-expect-error Ordering is restricted to the same phase
runtime.builder().before(math.blockRules[0], math.inlineRules[0]);
grammar.dispose();
const doc = core.parse("**check**");
const source: string = doc.source;
const kind: string | undefined = doc.children[0]?.kind;
const error: string = new MarkdownParseError({ code: "internal_error" }).detail
  .code;
const reference: ReferenceData = { type: "user", id: "u", label: "@u" };
const render = createRenderer({
  overrides: { [names.Link]: (node, ctx) => ctx.inline(node.children) },
  extensions,
});
const tokens = render.render(doc);
for (const parser of [core, runtime.parser(grammar, { allowUnknownNodes: false })]) {
  const node = parser.parseInline("[link](https://example.com)").children[0];
  if (node.kind === names.Link) {
    const destination: string = node.data.destination;
    void destination;
  }
}
function checkOpenNode(node: Node<true>) {
  if (node.kind === names.Link) {
    // @ts-expect-error A kind comparison cannot exclude unknown payloads.
    node.data.destination;
  }
  if (isKnownNode(node) && node.kind === names.Link) {
    const destination: string = node.data.destination;
    void destination;
    for (const child of node.children ?? []) {
      if (child.kind === names.Text) {
        // @ts-expect-error A known parent does not imply known descendants.
        child.data.value;
      }
      if (isKnownNode(child) && child.kind === names.Text) {
        const text: string = child.data.value;
        void text;
      }
    }
  }
}
const open = runtime.parser(grammar, { allowUnknownNodes: true });
const configured: ParserOptions = { allowUnknownNodes: Math.random() > 0.5 };
for (const parser of [open, runtime.parser(grammar, configured)]) {
  for (const result of [parser.parse("text"), parser.parseInline("text")]) {
    render.render(result);
    render.renderInline(result);
    checkOpenNode(result.children[0]);
    // @ts-expect-error Unknown nodes cannot be assigned to the closed union.
    const closed: Node = result.children[0];
  }
}
createRenderer({
  overrides: {
    [names.Link]: (node, ctx) => {
      if (isKnownNode(node) && node.kind === names.Link) {
        const destination: string = node.data.destination;
        void destination;
        return ctx.inline(node.children);
      }
      return ctx.fallback(node);
    },
    // @ts-expect-error Non-CommonMark handlers belong in extensions.
    [names.Stamp]: null,
  },
  extensions: new Map([["custom::Node", (node, ctx) => {
    // @ts-expect-error Arbitrary extension payloads need a check.
    node.data.value;
    return ctx.fallback(node);
  }]]),
});
void [source, kind, error, reference, tokens];
core.dispose();
runtime.dispose();
