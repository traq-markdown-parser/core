import {
  validateDocument,
  validateError,
  validateBuildError,
} from "./validate.mjs";
import { MarkdownParseError, GrammarBuildError } from "./errors.mjs";

export function createRunner(instance) {
  let wasm = instance.exports,
    usable = true;
  const arities = {
    abi_version: 0,
    ast_version: 0,
    input_ptr: 1,
    output_ptr: 0,
    parse: 2,
    grammar_build: 0,
    grammar_drop: 1,
    grammar_count: 0,
    contract_ptr: 0,
    contract_len: 0,
  };
  for (const [name, arity] of Object.entries(arities))
    if (typeof wasm[name] !== "function" || wasm[name].length !== arity)
      throw new Error("Invalid Wasm export: " + name);
  if (!(wasm.memory instanceof WebAssembly.Memory))
    throw new Error("Missing Wasm memory");
  if (wasm.abi_version() !== 2 || wasm.ast_version() !== 3)
    throw new Error("Unsupported Wasm ABI or AST schema");
  const encoder = new TextEncoder(),
    decoder = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });
  function read(pointer, length, max) {
    if (!Number.isInteger(length) || length <= 0 || length > max)
      throw new Error("Invalid Wasm output length");
    return JSON.parse(
      decoder.decode(new Uint8Array(wasm.memory.buffer, pointer, length)),
    );
  }
  const contract = read(wasm.contract_ptr(), wasm.contract_len(), 65536);
  if (
    contract.abiVersion !== 2 ||
    contract.astVersion !== 3 ||
    contract.limits?.inputBytes !== 65536 ||
    contract.limits?.outputBytes !== 1048576 ||
    contract.limits?.memoryBytes !== 33554432 ||
    contract.limits?.grammars !== 256
  )
    throw new Error("Invalid artifact contract");
  const freeze = (value) => {
    if (value && typeof value === "object") {
      Object.values(value).forEach(freeze);
      Object.freeze(value);
    }
  };
  freeze(contract);
  function invoke(name, args, source, field) {
    if (!usable || !wasm) throw new Error("Wasm instance is unavailable");
    if (typeof source !== "string")
      throw new TypeError("Expected source string");
    if (source.length > 65536)
      throw new MarkdownParseError({
        code: "resource_limit",
        resource: "input_bytes",
      });
    const input = encoder.encode(source);
    if (input.length > 65536)
      throw new MarkdownParseError({
        code: "resource_limit",
        resource: "input_bytes",
      });
    if (decoder.decode(input) !== source)
      throw new MarkdownParseError({ code: "invalid_utf8" });
    try {
      const pointer = wasm.input_ptr(input.length);
      if (!pointer) throw new Error("Wasm input allocation failed");
      new Uint8Array(wasm.memory.buffer, pointer, input.length).set(input);
      const length = wasm[name](...args);
      const result = read(wasm.output_ptr(), length, 1048576);
      if (Object.keys(result).length !== 1)
        throw new Error("Invalid Wasm result contract");
      if (result.error) {
        if (field === "grammar" && validateBuildError(result.error))
          throw new GrammarBuildError(result.error);
        if (validateError(result.error))
          throw new MarkdownParseError(result.error);
        throw new Error("Invalid Wasm error contract");
      }
      if (!Object.hasOwn(result, field)) throw new Error("Missing Wasm result");
      return result[field];
    } catch (error) {
      if (!(
        error instanceof MarkdownParseError ||
        error instanceof GrammarBuildError
      ))
        usable = false;
      throw error;
    }
  }
  return {
    contract,
    build(spec) {
      const result = invoke(
        "grammar_build",
        [],
        JSON.stringify(spec),
        "grammar",
      );
      if (
        !result ||
        !Number.isInteger(result.handle) ||
        result.handle <= 0 ||
        typeof result.description !== "string" ||
        !Array.isArray(result.extensions) ||
        !result.extensions.every((n) => typeof n === "string") ||
        new Set(result.extensions).size !== result.extensions.length
      ) {
        usable = false;
        throw new Error("Invalid grammar contract");
      }
      return result;
    },
    parse(handle, source, mode, extensions, allowUnknown) {
      const result = invoke("parse", [handle, mode], source, "document");
      try {
        return validateDocument(result, source, extensions, allowUnknown);
      } catch (error) {
        usable = false;
        throw error;
      }
    },
    drop(handle) {
      if (!usable || !wasm) return;
      if (wasm.grammar_drop(handle) !== 1) {
        usable = false;
        throw new Error("Invalid grammar release");
      }
    },
    dispose() {
      usable = false;
      wasm = null;
    },
  };
}
