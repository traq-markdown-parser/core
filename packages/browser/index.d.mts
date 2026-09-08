import type { Document } from "./generated/Document.js";
import type { ParseError } from "./generated/ParseError.js";
import type { CatalogViews } from "./generated/catalog.mjs";
import type {
  Plugin as Declaration,
  PluginGroup,
} from "@traptitech/markdown-definitions";
export type { Document, ParseError };
export type { Node } from "./generated/Node.js";
export type { NodeKind } from "./generated/NodeKind.js";
export { isKnownNode } from "./generated/nodes.mjs";
export type { ReferenceData } from "./generated/ReferenceData.js";

export interface Contract {
  readonly abiVersion: 2;
  readonly astVersion: 4;
  readonly catalog: unknown;
  readonly limits: Readonly<{
    inputBytes: number;
    outputBytes: number;
    memoryBytes: number;
    grammars: number;
  }>;
}
declare const identity: unique symbol;
export type Phase = "inline" | "block" | "text";
export interface Rule<P extends Phase = Phase> {
  readonly [identity]: P;
  readonly name: string | null;
  readonly phase: P;
}
export class Plugin {
  constructor(declaration: Declaration);
  readonly name: string;
  readonly namespace: PluginGroup | null;
  readonly inlineRules: readonly Rule<"inline">[];
  readonly blockRules: readonly Rule<"block">[];
  readonly textRules: readonly Rule<"text">[];
  add(rule: Rule): this;
}
export interface GrammarBuilder {
  add(plugin: Plugin): this;
  remove(plugin: Plugin): this;
  before(rule: Rule<"inline">, anchor: Rule<"inline">): this;
  before(rule: Rule<"block">, anchor: Rule<"block">): this;
  before(rule: Rule<"text">, anchor: Rule<"text">): this;
  /** Validate and compile this independent composition immediately. */
  build(): Grammar;
}
/** Immutable composition. Presets compile only on first parser construction. */
export interface Grammar {
  readonly [identity]: "grammar";
  readonly plugins: readonly Plugin[];
  toBuilder(): GrammarBuilder;
  describe(): string;
  dispose(): void;
}
export interface Parser<AllowUnknown extends boolean = false> {
  parse(source: string): Document<AllowUnknown>;
  parseInline(source: string): Document<AllowUnknown>;
  dispose(): void;
}
export interface ParserOptions<AllowUnknown extends boolean = boolean> {
  allowUnknownNodes?: AllowUnknown;
}
export interface RuntimeOptions {
  nodes?: ReadonlyMap<string, (data: unknown) => boolean>;
}
export interface Runtime extends CatalogViews {
  readonly contract: Contract;
  builder(): GrammarBuilder;
  /** Compile an unused preset, then share its compiled data with this parser. */
  parser<AllowUnknown extends boolean = false>(
    grammar: Grammar,
    options?: ParserOptions<AllowUnknown>,
  ): Parser<AllowUnknown>;
  dispose(): void;
}
export type BuildError =
  | { code: "duplicate" | "missing"; element: string }
  | { code: "duplicate_name"; scope: string; name: string }
  | { code: "invalid_definition"; reason: string };
export class MarkdownParseError extends Error {
  readonly detail: ParseError;
  constructor(error: ParseError);
}
export class GrammarBuildError extends Error {
  readonly detail: BuildError;
  constructor(error: BuildError);
}
export function loadRuntime(
  bytes: BufferSource,
  options?: RuntimeOptions,
): Promise<Runtime>;
