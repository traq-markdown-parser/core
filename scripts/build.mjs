import { execFileSync } from "node:child_process";
import { copyFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const target = path.resolve(root, process.env.CARGO_TARGET_DIR || "target");
const run = (command, args) =>
  execFileSync(command, args, {
    cwd: root,
    env: { ...process.env, CARGO_TARGET_DIR: target },
    stdio: "inherit",
    windowsHide: true,
  });

run("cargo", [
  "build", "--locked", "--release", "--target", "wasm32-unknown-unknown",
  "-p", "traq-markdown-wasm",
]);
run("cargo", [
  "run", "--locked", "--release", "-p", "traq-markdown-wasm", "--features", "contracts",
  "--bin", "export-node-contracts", "--", "packages/browser/generated",
]);
await copyFile(
  path.join(target, "wasm32-unknown-unknown/release/traq_markdown_wasm.wasm"),
  path.join(root, "packages/browser/parser.wasm"),
);
for (const [source, destination] of [
  ["THIRD_PARTY_NOTICES.md", "packages/browser/THIRD_PARTY_NOTICES.md"],
  ["LICENSE", "packages/browser/LICENSE"],
  ["LICENSE", "packages/renderer/LICENSE"],
]) {
  await copyFile(path.join(root, source), path.join(root, destination));
}
run(process.execPath, ["scripts/generate-bindings.mjs"]);
run(process.execPath, ["scripts/contract.mjs"]);
