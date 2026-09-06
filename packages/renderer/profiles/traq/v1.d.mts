import type { Handler, Renderer, RendererOptions } from "../../index.mjs";
import type { Parser } from "@traptitech/markdown-parser";
export const extensions: ReadonlyMap<string, Handler>;
export function createRenderer(options?: RendererOptions): Renderer;
export function installParser<
  T extends { md: { validateLink(destination: string): boolean } },
>(renderer: T, parser: Parser, options?: RendererOptions): T;
