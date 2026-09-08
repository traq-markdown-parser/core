import { names } from "@traptitech/markdown-parser/nodes";
import { token, pair, inlineToken } from "../tokens.mjs";
import { Plugin as Declaration } from "@traptitech/markdown-definitions";
import { Plugin } from "../plugin.mjs";

function math(node, block) {
  const t = token(
    block ? "math_block" : "math_inline",
    "math",
    0,
    node.data.tex,
  );
  t.block = block;
  return [t];
}
function table(node, ctx) {
  const row = (node) =>
    pair(
      "tr",
      "tr",
      (node.children ?? []).flatMap((cell) => {
        const tag = node.data.header ? "th" : "td";
        return pair(tag, tag, [inlineToken(ctx.inline(cell.children))], {
          block: true,
          attrs: cell.data.alignment
            ? [["style", "text-align:" + cell.data.alignment]]
            : null,
        });
      }),
      { block: true },
    );
  const head = (node.children ?? []).filter((r) => r.data.header),
    body = (node.children ?? []).filter((r) => !r.data.header);
  return pair(
    "table",
    "table",
    [
      ...pair("thead", "thead", head.flatMap(row), { block: true }),
      ...(body.length
        ? pair("tbody", "tbody", body.flatMap(row), { block: true })
        : []),
    ],
    { block: true },
  );
}
const declaration = Declaration.group("generic").new("presentation");
export function plugin() {
  const result = new Plugin(declaration);
  for (const [kind, handler] of [
    [names.Mark, (n, ctx) => pair("mark", "mark", ctx.inline(n.children))],
    [names.Strikethrough, (n, ctx) => pair("s", "s", ctx.inline(n.children))],
    [names.InlineMath, (n) => math(n, false)],
    [names.BlockMath, (n) => math(n, true)],
    [names.Table, table],
  ])
    result.on(kind, handler);
  return result;
}
