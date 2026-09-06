import type { Parser, Document, Node } from "@traptitech/markdown-parser";
import type Token from "markdown-it/lib/token.mjs";
export interface RenderContext {
  source: string;
  store: unknown;
  tight: boolean;
  validateLink(destination: string): boolean;
  inline(nodes?: Node[]): Token[];
  blocks(nodes?: Node[], tight?: boolean): Token[];
  fallback(node: Node): Token[];
}
export type Handler = (node: Node, context: RenderContext) => Token[];
export type CommonKind = Exclude<Node["kind"], "extension">;
export interface RendererOptions {
  store?: unknown;
  validateLink?(destination: string): boolean;
  overrides?: Partial<Record<CommonKind, Handler | null>>;
  extensions?: ReadonlyMap<string, Handler | null>;
}
export interface Renderer {
  render(document: Document): Token[];
  renderInline(document: Document): Token[];
}
export function createRenderer(options?: RendererOptions): Renderer;
export function installParser<
  T extends { md: { validateLink(destination: string): boolean } },
>(renderer: T, parser: Parser, options?: RendererOptions): T;
