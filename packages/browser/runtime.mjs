import { createRunner } from "./call.mjs";
import { GrammarBuilder, composition } from "./builder.mjs";
import { makeGrammar, makeParser } from "./grammar.mjs";
import { loadCatalog } from "./catalog.mjs";
import { extensions as defaults } from "./generated/extensions.mjs";
import { catalog as expectedCatalog } from "./generated/catalog.mjs";

export async function loadRuntime(bytes, options = {}) {
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const runner = createRunner(instance),
    contract = runner.contract;
  if (JSON.stringify(contract.catalog) !== JSON.stringify(expectedCatalog))
    throw new Error("Wasm catalog does not match this SDK");
  const extensions = new Map(options.extensions ?? defaults);
  for (const [name, validate] of extensions)
    if (typeof name !== "string" || typeof validate !== "function")
      throw new TypeError("Invalid extension decoder");
  const records = new Set();
  let disposed = false;
  function requireOpen() {
    if (disposed) throw new Error("Runtime is disposed");
  }
  function register(snapshot, { description, extensions, handle = null }) {
    requireOpen();
    const record = { snapshot, description, extensions, handle, references: 1 };
    records.add(record);
    return makeGrammar(owner, record);
  }
  const owner = {
    extensions,
    preset: register,
    build(snapshot) {
      requireOpen();
      return register(snapshot, runner.build(composition(snapshot)));
    },
    prepare(record) {
      requireOpen();
      if (record.handle !== null) return;
      const compiled = runner.build(composition(record.snapshot));
      if (
        compiled.description !== record.description ||
        JSON.stringify(compiled.extensions) !==
          JSON.stringify(record.extensions)
      ) {
        runner.dispose();
        throw new Error("Preset compilation does not match the catalog");
      }
      record.handle = compiled.handle;
    },
    parse(record, source, mode, allowUnknown) {
      requireOpen();
      return runner.parse(
        record.handle,
        source,
        mode,
        extensions,
        allowUnknown,
      );
    },
    release(record) {
      if (--record.references === 0) {
        records.delete(record);
        if (!disposed && record.handle !== null) runner.drop(record.handle);
      }
    },
  };
  let catalog;
  try {
    catalog = loadCatalog(owner, contract.catalog);
  } catch (error) {
    runner.dispose();
    throw error;
  }
  return Object.freeze({
    contract,
    ...catalog,
    builder() {
      requireOpen();
      return new GrammarBuilder(owner);
    },
    parser(grammar, options) {
      requireOpen();
      return makeParser(owner, grammar, options);
    },
    dispose() {
      if (disposed) return;
      disposed = true;
      try {
        for (const record of records)
          if (record.handle !== null) runner.drop(record.handle);
      } finally {
        records.clear();
        runner.dispose();
      }
    },
  });
}
