import type { Parser, Document, Node } from "@traptitech/markdown-parser";
import type { names } from "@traptitech/markdown-parser/nodes";
import type Token from "markdown-it/lib/token.mjs";
export interface RenderContext {
  source: string;
  store: unknown;
  tight: boolean;
  validateLink(destination: string): boolean;
  inline(nodes?: Node<true>[]): Token[];
  blocks(nodes?: Node<true>[], tight?: boolean): Token[];
  fallback(node: Node<true>): Token[];
}
export type Handler = (node: Node<true>, context: RenderContext) => Token[];
export type CommonKind = (typeof names)[
  | "Blockquote" | "CodeBlock" | "Emphasis" | "Hardbreak" | "Heading"
  | "HtmlBlock" | "HtmlInline" | "Image" | "InlineCode" | "Link"
  | "List" | "ListItem" | "Paragraph" | "Softbreak" | "Strong" | "Text"
  | "ThematicBreak"
];
export interface RendererOptions {
  store?: unknown;
  validateLink?(destination: string): boolean;
  overrides?: Partial<Record<CommonKind, Handler | null>>;
  extensions?: ReadonlyMap<string, Handler | null>;
}
export interface Renderer {
  render(document: Document<true>): Token[];
  renderInline(document: Document<true>): Token[];
}
export function createRenderer(options?: RendererOptions): Renderer;
export function installParser<
  T extends { md: { validateLink(destination: string): boolean } },
>(renderer: T, parser: Parser<boolean>, options?: RendererOptions): T;
