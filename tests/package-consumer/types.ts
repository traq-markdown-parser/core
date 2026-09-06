import {
  MarkdownParseError,
  type ReferenceData,
} from "@traptitech/markdown-parser";
import { loadRuntime, Plugin } from "@traptitech/markdown-parser";
import { createRenderer } from "@traptitech/markdown-renderer";
import { extensions } from "@traptitech/markdown-renderer/traq/v1";
const runtime = await loadRuntime(new Uint8Array());
const grammar = runtime.presets.traq.v1
  .toBuilder()
  .remove(runtime.plugins.generic.math)
  .build();
const core = runtime.parser(grammar);
const generic = Plugin.group().named("custom");
const plugin = generic
  .new()
  .named("math")
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
  overrides: { link: (node, ctx) => ctx.inline(node.children) },
  extensions,
});
const tokens = render.render(doc);
void [source, kind, error, reference, tokens];
core.dispose();
runtime.dispose();
