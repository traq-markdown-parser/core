import { metadata, owned } from "./identity.mjs";
import { makePlugin } from "./plugin.mjs";
import { GrammarBuilder } from "./builder.mjs";

export function makeGrammar(runtime, record) {
  const grammar = {
    plugins: Object.freeze(
      record.snapshot.plugins.map((state) =>
        makePlugin({ ...state, frozen: true }),
      ),
    ),
    toBuilder: () => new GrammarBuilder(runtime, record.snapshot),
    describe: () => record.description,
    dispose() {
      const state = owned(grammar, "grammar", runtime);
      if (!state.disposed) {
        state.disposed = true;
        runtime.release(record);
      }
    },
  };
  metadata.set(grammar, { kind: "grammar", runtime, record, disposed: false });
  return Object.freeze(grammar);
}
export function makeParser(runtime, grammar, options) {
  const state = owned(grammar, "grammar", runtime);
  if (state.disposed) throw new Error("Grammar is disposed");
  const record = state.record;
  const allowUnknownNodes = options?.allowUnknownNodes ?? false;
  if (typeof allowUnknownNodes !== "boolean")
    throw new TypeError("Expected boolean allowUnknownNodes");
  runtime.prepare(record);
  record.references++;
  let disposed = false;
  const parse = (source, mode) => {
    if (disposed) throw new Error("Parser is disposed");
    return runtime.parse(record, source, mode, allowUnknownNodes);
  };
  return Object.freeze({
    parse: (source) => parse(source, 0),
    parseInline: (source) => parse(source, 1),
    dispose() {
      if (!disposed) {
        disposed = true;
        runtime.release(record);
      }
    },
  });
}
