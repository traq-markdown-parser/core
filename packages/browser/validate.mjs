import {
  fields,
  string,
  boolean,
  uint,
  nullable,
  oneOf,
  object,
} from "./fields.mjs";

const common = new Map([
  ["paragraph", {}],
  ["blockquote", {}],
  ["emphasis", {}],
  ["strong", {}],
  ["heading", { level: (value) => uint(value) && value >= 1 && value <= 6 }],
  ["list", { ordered: boolean, start: uint, tight: boolean }],
  ["list_item", { marker: string }],
  ["code_block", { fenced: boolean, info: string, literal: string }],
  ["thematic_break", { marker: string }],
  ["text", { value: string }],
  ["softbreak", {}],
  ["hardbreak", {}],
  ["inline_code", { literal: string }],
  [
    "link",
    {
      destination: string,
      title: nullable(string),
      form: oneOf("explicit", "autolink", "linkify"),
    },
  ],
  [
    "image",
    { destination: string, title: nullable(string), label_source: string },
  ],
  ["html_inline", { literal: string }],
  ["html_block", { literal: string }],
  ["extension", { name: string, data: object }],
]);
const containers = new Set([
  "paragraph",
  "blockquote",
  "heading",
  "list",
  "list_item",
  "emphasis",
  "strong",
  "link",
  "image",
  "extension",
]);

export function validateDocument(
  document,
  source,
  extensions,
  allowUnknown = false,
) {
  if (
    !fields(document, {
      source: (value) => value === source,
      children: Array.isArray,
    })
  )
    throw new Error("Invalid document contract");
  const bytes = new TextEncoder().encode(source);
  const boundary = (offset) =>
    uint(offset) &&
    offset <= bytes.length &&
    (offset === bytes.length || (bytes[offset] & 0xc0) !== 0x80);
  const pending = document.children.map((node) => [node, 0, bytes.length, 1]);
  let count = 0;
  while (pending.length) {
    const [node, start, end, depth] = pending.pop();
    const shape = common.get(node?.kind);
    if (
      ++count > 16384 ||
      depth > 64 ||
      !shape ||
      !fields(
        node,
        {
          kind: string,
          span: (span) => fields(span, { start: boundary, end: boundary }),
          ...shape,
        },
        { children: Array.isArray },
      )
    )
      throw new Error("Invalid AST node contract");
    if (
      node.span.start < start ||
      node.span.end > end ||
      node.span.start > node.span.end
    )
      throw new Error("Invalid AST span");
    if (node.children?.length && !containers.has(node.kind))
      throw new Error("Unexpected AST children");
    if (node.kind === "extension") {
      const decode = extensions.get(node.name);
      if (decode ? !decode(node.data) : !allowUnknown)
        throw new Error(`Unsupported or invalid extension: ${node.name}`);
    }
    for (const child of node.children ?? [])
      pending.push([child, node.span.start, node.span.end, depth + 1]);
  }
  return document;
}

export function validateError(error) {
  const shapes = {
    invalid_utf8: {},
    internal_error: {},
    resource_limit: { resource: string },
  };
  return (
    object(error) &&
    Object.hasOwn(shapes, error.code) &&
    fields(error, { code: string, ...shapes[error.code] })
  );
}

export function validateBuildError(error) {
  const shapes = {
    duplicate: { element: string },
    missing: { element: string },
    duplicate_name: { scope: string, name: string },
    invalid_definition: { reason: string },
  };
  return (
    object(error) &&
    Object.hasOwn(shapes, error.code) &&
    fields(error, { code: string, ...shapes[error.code] })
  );
}
