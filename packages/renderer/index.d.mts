import type { Plugin as Declaration } from "@traptitech/markdown-definitions";
import type Token from "markdown-it/lib/token.mjs";

/** Structural AST input; grammar packages own the payload types. */
export interface Node {
  kind: string;
  span: { start: number; end: number };
  data: unknown;
  children?: Node[];
}
export interface Document {
  source: string;
  children: Node[];
}
export interface RenderContext {
  source: string;
  inline(nodes?: Node[]): Token[];
  blocks(nodes?: Node[]): Token[];
  fallback(node: Node): Token[];
}
export type Handler = (node: Node, context: RenderContext) => Token[];
export class Plugin {
  constructor(declaration: Declaration);
  on(kind: string, handler: Handler): this;
  replace(kind: string, handler: Handler): this;
}
declare const identity: unique symbol;
export interface Preset {
  readonly [identity]: "renderer-preset";
}
export class PresetBuilder {
  add(plugin: Plugin): this;
  remove(plugin: Plugin): this;
  build(): Preset;
}
export interface Renderer {
  render(document: Document): Token[];
  renderInline(document: Document): Token[];
}
export function renderer(preset: Preset): Renderer;
export function installParser<T extends { md: object }>(
  target: T,
  parser: {
    parse(source: string): Document;
    parseInline(source: string): Document;
  },
  renderer: Renderer,
): T;
