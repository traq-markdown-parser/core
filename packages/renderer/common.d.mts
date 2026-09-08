import type { Plugin, Preset } from "./index.mjs";
import type { names } from "@traptitech/markdown-parser/nodes";
export interface Options {
  validateLink?(destination: string): boolean;
}
export const nodes: Pick<
  typeof names,
  | "Blockquote"
  | "CodeBlock"
  | "Emphasis"
  | "Hardbreak"
  | "Heading"
  | "HtmlBlock"
  | "HtmlInline"
  | "Image"
  | "InlineCode"
  | "Link"
  | "List"
  | "ListItem"
  | "Paragraph"
  | "Softbreak"
  | "Strong"
  | "Text"
  | "ThematicBreak"
>;
export function plugin(options?: Options): Plugin;
export const html: Readonly<{ plugin: typeof plugin }>;
export function preset(options?: Options): Preset;
