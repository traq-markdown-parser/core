import { names } from "@traptitech/markdown-parser/nodes";
import { token, pair } from "../tokens.mjs";

function reference(node, ctx) {
  const { type, id, label } = node.data,
    store = ctx.store;
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
export const trap = new Map([
  [names.Reference, reference],
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
]);
