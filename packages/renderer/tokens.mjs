import Token from "markdown-it/lib/token.mjs";

export function token(type, tag = "", nesting = 0, content = "") {
  const value = new Token(type, tag, nesting);
  value.content = content;
  return value;
}

export function pair(
  type,
  tag,
  children,
  { block = false, attrs = null, markup = "", hidden = false, info = "" } = {},
) {
  const open = token(type + "_open", tag, 1),
    close = token(type + "_close", tag, -1);
  Object.assign(open, { block, attrs, markup, hidden, info });
  Object.assign(close, { block, markup, hidden });
  return [open, ...children, close];
}

export function inlineToken(children) {
  const value = token("inline");
  value.children = children;
  return value;
}
