import { execFileSync } from "node:child_process";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { javascript } from "./contracts/javascript.mjs";
import { goPayload, goRegistry } from "./contracts/go.mjs";
import { catalogFiles } from "./contracts/catalog.mjs";
import { typeName } from "./contracts/schema.mjs";
import { treeFiles } from "./contracts/tree.mjs";
const manifest = JSON.parse(
  await readFile(
    new URL("../packages/browser/generated/contracts.json", import.meta.url),
  ),
);
const entries = Object.entries(manifest.nodes).map(([key, entry]) => [key, entry.schema]);
const files = new Map();
for (const [p, s] of treeFiles(entries)) files.set(p, s);
files.set("packages/browser/generated/nodes.mjs", javascript(entries));
files.set(
  "packages/browser/generated/nodes.d.mts",
  "import type { Node } from './Node.js';\n" +
    "import type { NodeKind } from './NodeKind.js';\n" +
    "export const names: Readonly<{" +
    entries
      .map(([name, s]) => typeName(s) + ":" + JSON.stringify(name))
      .join(";") +
    "}>\nexport const nodes:ReadonlyMap<string,(data:unknown)=>boolean>\n" +
    "export function isKnownNode(node: Node<true>): node is Node<true> & NodeKind;\n",
);
const groups = Map.groupBy(entries, ([name]) => manifest.nodes[name].group);
for (const [group, entries] of groups) {
  for (const [name, schema] of entries)
    files.set(
      "go/extensions/" + group + "/" + typeName(schema).toLowerCase() + ".go",
      goPayload(group, name, schema),
    );
  files.set(
    "go/extensions/" + group + "/registry.go",
    goRegistry(group, entries),
  );
}
for (const [p, s] of catalogFiles(manifest.catalog, [...groups.keys()]))
  files.set(p, s);
for (const [p, s] of files) {
  await mkdir(path.dirname(p), { recursive: true });
  await writeFile(p, s);
}
execFileSync(
  "gofmt",
  ["-w", ...[...files.keys()].filter((p) => p.endsWith(".go"))],
  { windowsHide: true },
);
console.log(
  "Generated " +
    files.size +
    " host binding files from " +
    entries.length +
    " Rust payloads",
);
