import { token, pair, inlineToken } from "./tokens.mjs";

export const common = new Map([
  ["text", (n) => [token("text", "", 0, n.value)]],
  ["inline_code", (n) => [token("code_inline", "code", 0, n.literal)]],
  ["softbreak", () => [token("softbreak", "br")]],
  ["hardbreak", () => [token("hardbreak", "br")]],
  ["emphasis", (n, ctx) => pair("em", "em", ctx.inline(n.children))],
  ["strong", (n, ctx) => pair("strong", "strong", ctx.inline(n.children))],
  [
    "link",
    (n, ctx) => {
      if (!ctx.validateLink(n.destination)) return ctx.inline(n.children);
      const attrs = [["href", n.destination]];
      if (n.title !== null) attrs.push(["title", n.title]);
      return pair("link", "a", ctx.inline(n.children), {
        attrs,
        markup: n.form === "explicit" ? "" : n.form,
      });
    },
  ],
  [
    "image",
    (n, ctx) => {
      if (!ctx.validateLink(n.destination)) return ctx.fallback(n);
      const t = token("image", "img", 0, n.label_source);
      t.attrs = [
        ["src", n.destination],
        ["alt", ""],
      ];
      if (n.title !== null) t.attrs.push(["title", n.title]);
      t.children = ctx.inline(n.children);
      return [t];
    },
  ],
  [
    "paragraph",
    (n, ctx) =>
      pair("paragraph", "p", [inlineToken(ctx.inline(n.children))], {
        block: true,
        hidden: ctx.tight,
      }),
  ],
  [
    "heading",
    (n, ctx) =>
      pair("heading", `h${n.level}`, [inlineToken(ctx.inline(n.children))], {
        block: true,
      }),
  ],
  [
    "blockquote",
    (n, ctx) =>
      pair("blockquote", "blockquote", ctx.blocks(n.children), {
        block: true,
        markup: ">",
      }),
  ],
  [
    "list",
    (n, ctx) =>
      pair(
        n.ordered ? "ordered_list" : "bullet_list",
        n.ordered ? "ol" : "ul",
        ctx.blocks(n.children, n.tight),
        {
          block: true,
          attrs:
            n.ordered && n.start !== 1 ? [["start", String(n.start)]] : null,
        },
      ),
  ],
  [
    "list_item",
    (n, ctx) =>
      pair("list_item", "li", ctx.blocks(n.children, ctx.tight), {
        block: true,
        markup: n.marker.slice(-1),
        info: n.marker.slice(0, -1),
      }),
  ],
  [
    "code_block",
    (n) => {
      const t = token(n.fenced ? "fence" : "code_block", "code", 0, n.literal);
      Object.assign(t, { info: n.info, block: true });
      return [t];
    },
  ],
  [
    "thematic_break",
    (n) => {
      const t = token("hr", "hr");
      Object.assign(t, { block: true, markup: n.marker });
      return [t];
    },
  ],
  // Recognition of HTML never implies permission to execute it.
  ["html_inline", (n) => [token("text", "", 0, n.literal)]],
  [
    "html_block",
    (n) =>
      pair("paragraph", "p", [inlineToken([token("text", "", 0, n.literal)])], {
        block: true,
      }),
  ],
]);
