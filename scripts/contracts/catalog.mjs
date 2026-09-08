import { quoted as q } from "./schema.mjs";
const goName = (name) =>
  ({ traq: "TraQ", commonmark: "CommonMark" })[name] ??
  name[0].toUpperCase() + name.slice(1);
function tsTree(tree, leaf) {
  if (typeof tree === "number") return leaf;
  return (
    "{ " +
    Object.entries(tree)
      .map(
        ([name, child]) => "readonly " + q(name) + ": " + tsTree(child, leaf),
      )
      .join("; ") +
    " }"
  );
}
export function goTree(tree, leaf) {
  if (typeof tree === "number") return "*" + (typeof leaf === "function" ? leaf(tree) : leaf);
  return (
    "struct {\n" +
    Object.entries(tree)
      .map(([name, child]) => goName(name) + " " + goTree(child, leaf))
      .join("\n") +
    "\n}"
  );
}
export function assignments(tree, root, leaf) {
  if (typeof tree === "number") return root + " = " +
    (typeof leaf === "function" ? leaf(tree) : leaf + "[" + tree + "]") + "\n";
  return Object.entries(tree)
    .map(([name, child]) => assignments(child, root + "." + goName(name), leaf))
    .join("");
}
export function catalogFiles(catalog, groups) {
  return new Map([
    [
      "packages/browser/generated/catalog.mjs",
      "// Generated from the Rust distribution catalog.\nexport const catalog = " +
        q(catalog) +
        "\n",
    ],
    [
      "packages/browser/generated/catalog.d.mts",
      "import type {Grammar,Plugin} from '../index.mjs'\nexport const catalog: unknown\nexport interface CatalogViews {\n" +
        "readonly plugins: " +
        tsTree(catalog.exports.plugins, "Plugin") +
        "\nreadonly presets: " +
        tsTree(catalog.exports.presets, "Grammar") +
        "\n}\n",
    ],
    ["go/binding/catalog.json", JSON.stringify(catalog, null, 2) + "\n"],
    [
      "go/binding/generated.go",
      "// Code generated from the Rust catalog. DO NOT EDIT.\npackage binding\n" +
        'import (_ "embed"; "github.com/traPtitech/traq-markdown-parser/go/ast";' +
        groups
          .map((g) =>
            q("github.com/traPtitech/traq-markdown-parser/go/extensions/" + g),
          )
          .join(";") +
        ")\n" +
        "//go:embed catalog.json\nvar CatalogJSON []byte\n" +
        "func Registry() ast.Registry {all:=ast.Registry{}\n" +
        groups
          .map(
            (g) =>
              "for name,decoder:=range " + g + ".Registry(){all[name]=decoder}",
          )
          .join("\n") +
        "\nreturn all}\n",
    ],
    [
      "go/core/catalog_generated.go",
      "// Code generated from the Rust SDK exports. DO NOT EDIT.\npackage core\n" +
        "type Plugins = " +
        goTree(catalog.exports.plugins, "Plugin") +
        "\n" +
        "type Presets = " +
        goTree(catalog.exports.presets, "Grammar") +
        "\n" +
        "func (r *Runtime) exportCatalog(plugins []*Plugin,presets []*Grammar) {\n" +
        assignments(catalog.exports.plugins, "r.Plugins", "plugins") +
        assignments(catalog.exports.presets, "r.Presets", "presets") +
        "}\n",
    ],
  ]);
}
