import { readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { gzipSync, brotliCompressSync } from "node:zlib";
import { loadRuntime } from "../packages/browser/index.mjs";
const bytes = await readFile(
  new URL("../packages/browser/parser.wasm", import.meta.url),
);
const core = await loadRuntime(bytes);
const contract = {
  ...core.contract,
  sha256: createHash("sha256").update(bytes).digest("hex"),
  bytes: bytes.length,
  gzipBytes: gzipSync(bytes).length,
  brotliBytes: brotliCompressSync(bytes).length,
};
await writeFile(
  new URL("../packages/browser/contract.json", import.meta.url),
  JSON.stringify(contract, null, 2) + "\n",
);
console.log(JSON.stringify(contract));
