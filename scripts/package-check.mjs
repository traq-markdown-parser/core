// Pack locally, then test a fresh consumer outside this workspace.
import {
  mkdir,
  mkdtemp,
  readFile,
  writeFile,
  copyFile,
  realpath,
  rm,
} from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import { fileURLToPath } from "node:url";
import path from "node:path";

const root = fileURLToPath(new URL("../", import.meta.url));
const npmCli = process.env.npm_execpath;
if (!npmCli) throw new Error("Run npm run check:package");
const run = (args, cwd = root) =>
  execFileSync(process.execPath, args, {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });
const npm = (args, cwd) => run([npmCli, ...args], cwd);
const dist = path.join(root, "dist");
await mkdir(dist, { recursive: true });
const archives = ["browser", "renderer"].map((name) => {
  const packed = JSON.parse(
    npm(
      ["pack", "--ignore-scripts", "--json", "--pack-destination", dist],
      path.join(root, "packages", name),
    ),
  );
  if (!packed[0].files.some((file) => file.path === "LICENSE"))
    throw new Error("Packed package is missing its LICENSE: " + name);
  return path.join(dist, packed[0].filename);
});

const tempRoot = await realpath(tmpdir());
const directory = await mkdtemp(path.join(tempRoot, "traq-markdown-package-"));
try {
  await writeFile(
    path.join(directory, "package.json"),
    JSON.stringify({ private: true, type: "module" }),
  );
  const workspace = JSON.parse(await readFile(path.join(root, "package.json")));
  npm(
    [
      "install",
      "--offline",
      "--ignore-scripts",
      "--no-audit",
      "--no-fund",
      ...archives,
      "markdown-it@" + workspace.devDependencies["markdown-it"],
    ],
    directory,
  );
  for (const file of ["types.ts", "runtime.mjs"])
    await copyFile(
      path.join(root, "tests/package-consumer", file),
      path.join(directory, file),
    );
  const tsc = fileURLToPath(import.meta.resolve("typescript/bin/tsc"));
  run(
    [
      tsc,
      "--noEmit",
      "--strict",
      "--target",
      "ES2022",
      "--module",
      "NodeNext",
      "--moduleResolution",
      "NodeNext",
      "types.ts",
    ],
    directory,
  );
  const contract = JSON.parse(
    await readFile(path.join(root, "packages/browser/contract.json")),
  );
  process.stdout.write(run(["runtime.mjs", contract.sha256], directory));
} finally {
  // Only remove the exact temporary consumer created above.
  if (path.dirname(path.resolve(directory)) !== tempRoot)
    throw new Error("Unexpected temporary consumer path");
  await rm(directory, { recursive: true, force: true });
}
