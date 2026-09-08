import { common } from "./common.mjs";
import { validateLink as defaultPolicy } from "./policy.mjs";
import { token, inlineToken, pair } from "./tokens.mjs";

// This adapter preserves S-UI's presentation plugins. It never parses Markdown.
export function createRenderer({
  store,
  validateLink = defaultPolicy,
  overrides = {},
  extensions = new Map(),
} = {}) {
  const handlers = new Map(common),
    extensionHandlers = new Map(extensions);
  for (const [kind, handler] of Object.entries(overrides)) {
    if (handler !== null && typeof handler !== "function")
      throw new TypeError("Expected render handler");
    if (!handlers.has(kind)) throw new Error("Unknown common node: " + kind);
    handlers.set(kind, handler);
  }
  for (const handler of extensionHandlers.values())
    if (handler !== null && typeof handler !== "function")
      throw new TypeError("Expected extension handler");
  for (const [kind, handler] of extensionHandlers) handlers.set(kind, handler);
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
    function nodes(values = [], block = false, tight = false) {
      const ctx = {
        store,
        validateLink,
        tight,
        source: document.source,
        inline: (values) => nodes(values),
        blocks: (values, tight = false) => nodes(values, true, tight),
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

export function installParser(renderer, parser, options = {}) {
  const render = createRenderer({
    validateLink: (value) => renderer.md.validateLink(value),
    ...options,
  });
  renderer.md.parse = (source) => render.render(parser.parse(source));
  renderer.md.parseInline = (source) =>
    render.renderInline(parser.parseInline(source));
  return renderer;
}
