import { execFileSync } from "node:child_process";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { javascript } from "./contracts/javascript.mjs";
import { goPayload, goRegistry } from "./contracts/go.mjs";
import { catalogFiles } from "./contracts/catalog.mjs";
import { typeName } from "./contracts/schema.mjs";
const manifest = JSON.parse(
  await readFile(
    new URL("../packages/browser/generated/contracts.json", import.meta.url),
  ),
);
const entries = Object.entries(manifest.extensions);
const files = new Map();
files.set("packages/browser/generated/extensions.mjs", javascript(entries));
files.set(
  "packages/browser/generated/extensions.d.mts",
  "export const names: Readonly<{" +
    entries
      .map(([name, s]) => typeName(s) + ":" + JSON.stringify(name))
      .join(";") +
    "}>\nexport const extensions:ReadonlyMap<string,(data:unknown)=>boolean>\n",
);
const groups = Map.groupBy(entries, ([name]) => name.split("/")[0]);
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
