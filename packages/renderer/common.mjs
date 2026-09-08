import { names } from "@traptitech/markdown-parser/nodes";
import { token, pair, inlineToken } from "./tokens.mjs";
import { Plugin as Declaration } from "@traptitech/markdown-definitions";
import { Plugin } from "./plugin.mjs";
import { PresetBuilder } from "./preset.mjs";
import { validateLink as defaultPolicy } from "./policy.mjs";
import { listChildren } from "./list.mjs";

const declaration = Declaration.group("commonmark").new("core");
export const nodes = Object.freeze(
  Object.fromEntries(
    [
      "Blockquote",
      "CodeBlock",
      "Emphasis",
      "Hardbreak",
      "Heading",
      "HtmlBlock",
      "HtmlInline",
      "Image",
      "InlineCode",
      "Link",
      "List",
      "ListItem",
      "Paragraph",
      "Softbreak",
      "Strong",
      "Text",
      "ThematicBreak",
    ].map((name) => [name, names[name]]),
  ),
);
export function plugin({ validateLink = defaultPolicy } = {}) {
  const result = new Plugin(declaration);
  for (const [kind, handler] of [
    [names.Text, (n) => [token("text", "", 0, n.data.value)]],
    [
      names.InlineCode,
      (n) => [token("code_inline", "code", 0, n.data.literal)],
    ],
    [names.Softbreak, () => [token("softbreak", "br")]],
    [names.Hardbreak, () => [token("hardbreak", "br")]],
    [names.Emphasis, (n, ctx) => pair("em", "em", ctx.inline(n.children))],
    [
      names.Strong,
      (n, ctx) => pair("strong", "strong", ctx.inline(n.children)),
    ],
    [
      names.Link,
      (n, ctx) => {
        if (!validateLink(n.data.destination)) return ctx.inline(n.children);
        const attrs = [["href", n.data.destination]];
        if (n.data.title !== null) attrs.push(["title", n.data.title]);
        return pair("link", "a", ctx.inline(n.children), {
          attrs,
          markup: n.data.form === "explicit" ? "" : n.data.form,
        });
      },
    ],
    [
      names.Image,
      (n, ctx) => {
        if (!validateLink(n.data.destination)) return ctx.fallback(n);
        const t = token("image", "img", 0, n.data.label_source);
        t.attrs = [
          ["src", n.data.destination],
          ["alt", ""],
        ];
        if (n.data.title !== null) t.attrs.push(["title", n.data.title]);
        t.children = ctx.inline(n.children);
        return [t];
      },
    ],
    [
      names.Paragraph,
      (n, ctx) =>
        pair("paragraph", "p", [inlineToken(ctx.inline(n.children))], {
          block: true,
        }),
    ],
    [
      names.Heading,
      (n, ctx) =>
        pair(
          "heading",
          `h${n.data.level}`,
          [inlineToken(ctx.inline(n.children))],
          {
            block: true,
          },
        ),
    ],
    [
      names.Blockquote,
      (n, ctx) =>
        pair("blockquote", "blockquote", ctx.blocks(n.children), {
          block: true,
          markup: ">",
        }),
    ],
    [
      names.List,
      (n, ctx) =>
        pair(
          n.data.ordered ? "ordered_list" : "bullet_list",
          n.data.ordered ? "ol" : "ul",
          listChildren(n, ctx),
          {
            block: true,
            attrs:
              n.data.ordered && n.data.start !== 1
                ? [["start", String(n.data.start)]]
                : null,
          },
        ),
    ],
    [
      names.ListItem,
      (n, ctx) =>
        pair("list_item", "li", ctx.blocks(n.children), {
          block: true,
          markup: n.data.marker.slice(-1),
          info: n.data.marker.slice(0, -1),
        }),
    ],
    [
      names.CodeBlock,
      (n) => {
        const t = token(
          n.data.fenced ? "fence" : "code_block",
          "code",
          0,
          n.data.literal,
        );
        Object.assign(t, { info: n.data.info, block: true });
        return [t];
      },
    ],
    [
      names.ThematicBreak,
      (n) => {
        const t = token("hr", "hr");
        Object.assign(t, { block: true, markup: n.data.marker });
        return [t];
      },
    ],
    // Recognition of HTML never implies permission to execute it.
    [names.HtmlInline, (n) => [token("text", "", 0, n.data.literal)]],
    [
      names.HtmlBlock,
      (n) =>
        pair(
          "paragraph",
          "p",
          [inlineToken([token("text", "", 0, n.data.literal)])],
          {
            block: true,
          },
        ),
    ],
  ])
    result.on(kind, handler);
  return result;
}
export const html = Object.freeze({ plugin });
export function preset(options) {
  return new PresetBuilder().add(plugin(options)).build();
}
