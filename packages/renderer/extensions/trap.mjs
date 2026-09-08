import { names } from "@traptitech/markdown-parser/nodes";
import { token, pair } from "../tokens.mjs";
import { Plugin as Declaration } from "@traptitech/markdown-definitions";
import { Plugin } from "../plugin.mjs";

function reference(node, store) {
  const { type, id, label } = node.data;
  if (!store) return [token("text", "", 0, label)];
  const me = store.getMe();
  const href =
    type === "user"
      ? store.generateUserHref(id)
      : type === "group"
        ? store.generateUserGroupHref(id)
        : store.generateChannelHref(id);
  const highlight =
    type === "user"
      ? id === me?.id
      : type === "group" &&
        (store.getUserGroup(id)?.members?.some((u) => u.id === me?.id) ??
          false);
  const cls = `message-${type}-link`;
  const tokens = pair("traq_extends_link", "a", [token("text", "", 0, label)], {
    attrs: [
      ["href", href],
      ["class", highlight ? `${cls}-highlight ${cls}` : cls],
    ],
  });
  tokens[0].meta = { type, data: type === "channel" ? label : id };
  return tokens;
}
const declaration = Declaration.group("trap").new("presentation");
export function plugin({ store } = {}) {
  const result = new Plugin(declaration);
  for (const [kind, handler] of [
    [names.Reference, (node) => reference(node, store)],
    [
      names.Stamp,
      (n) => {
        const t = token("regexp-0");
        t.meta = { match: [n.data.literal] };
        return [t];
      },
    ],
    [
      names.Spoiler,
      (n, ctx) =>
        pair("spoiler", "span", ctx.inline(n.children), {
          attrs: [["class", "spoiler"]],
          markup: "!!",
        }),
    ],
    [names.BlankLine, () => [token("hardbreak", "br")]],
  ])
    result.on(kind, handler);
  return result;
}
