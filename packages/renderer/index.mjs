import { handlers as presetHandlers } from "./preset.mjs";
import { token, inlineToken, pair } from "./tokens.mjs";
export { Plugin } from "./plugin.mjs";
export { PresetBuilder } from "./preset.mjs";

// This adapter preserves S-UI's presentation plugins. It never parses Markdown.
export function renderer(preset) {
  const handlers = presetHandlers(preset);
  function render(document, inline) {
    let bytes;
    const fallback = (node, block) => {
      bytes ??= new TextEncoder().encode(document.source);
      const text = new TextDecoder().decode(
        bytes.subarray(node.span.start, node.span.end),
      );
      const tokens = [token("text", "", 0, text)];
      return block
        ? pair("paragraph", "p", [inlineToken(tokens)], { block: true })
        : tokens;
    };
    function nodes(values = [], block = false) {
      const ctx = {
        source: document.source,
        inline: (values) => nodes(values),
        blocks: (values) => nodes(values, true),
        fallback: (node) => fallback(node, block),
      };
      return values.flatMap((node) => {
        const handler = handlers.get(node.kind);
        return handler ? handler(node, ctx) : fallback(node, block);
      });
    }
    return inline
      ? [inlineToken(nodes(document.children))]
      : nodes(document.children, true);
  }
  return Object.freeze({
    render: (document) => render(document, false),
    renderInline: (document) => render(document, true),
  });
}

export function installParser(target, parser, render) {
  target.md.parse = (source) => render.render(parser.parse(source));
  target.md.parseInline = (source) =>
    render.renderInline(parser.parseInline(source));
  return target;
}
