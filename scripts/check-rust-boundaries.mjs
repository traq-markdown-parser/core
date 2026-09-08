import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { dirname, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const result = spawnSync('cargo', ['metadata', '--offline', '--locked', '--format-version', '1'], {
  cwd: root, encoding: 'utf8', windowsHide: true, maxBuffer: 16 * 1024 * 1024,
});
if (result.error || result.status !== 0) {
  throw result.error ?? new Error(result.stderr);
}
const metadata = JSON.parse(result.stdout);
const packages = new Map(metadata.packages.map(pkg => [pkg.id, pkg]));
const graph = new Map(metadata.resolve.nodes.map(node => [node.id, node.deps
  .filter(dep => dep.dep_kinds.some(kind => kind.kind === null))
  .map(dep => dep.pkg)]));

function dependencies(id, visited = new Set()) {
  if (visited.has(id)) return visited;
  visited.add(id);
  for (const next of graph.get(id) ?? []) dependencies(next, visited);
  return visited;
}

// Directories define release groups; each remains a set of Cargo packages.
const groupOf = pkg => relative(root, pkg.manifest_path).replaceAll('\\', '/')
  .match(/^crates\/(core|commonmark|trap)\//)?.[1];
const allowed = {
  core: new Set(['core']),
  commonmark: new Set(['core', 'commonmark']),
  trap: new Set(['core', 'commonmark', 'trap']),
};
const members = metadata.workspace_members.map(id => packages.get(id));
for (const [group, permitted] of Object.entries(allowed)) {
  const selected = members.filter(pkg => groupOf(pkg) === group);
  assert.ok(selected.length, `Missing release group: ${group}`);
  assert.equal(new Set(selected.map(pkg => pkg.version)).size, 1,
    `${group}: package versions must match within this release group`);
  for (const pkg of selected) {
    for (const id of dependencies(pkg.id)) {
      const dependency = packages.get(id);
      if (dependency.source !== null) continue;
      assert.ok(permitted.has(groupOf(dependency)),
        `${pkg.name} has a dependency outside its permitted groups: ${dependency.name}`);
    }
  }
  console.log(`${group}: ${selected.length} packages, version ${selected[0].version}, group boundary passed`);
}

// Include transitive dependencies, so moving a forbidden import into a helper
// package cannot hide it. Dev dependencies may use richer fixtures and presets.
const boundaries = {
  'traq-markdown-wasm': /^traq-markdown$/,
  'markdown-traq-processing': /^markdown-(?:parser|codec|commonmark$|generic-syntax|trap-syntax|traq$)/,
  'markdown-trap-extraction': /^markdown-(?:parser|renderer|codec|commonmark|generic|traq|trap-text|trap-syntax)/,
  'markdown-commonmark-text': /^markdown-(?:generic|trap|traq|codec|parser|extractor|commonmark$)/,
  'markdown-generic-text': /^markdown-(?:commonmark|trap|traq|codec|parser|extractor|generic-syntax)/,
  'markdown-trap-text': /^markdown-(?:commonmark|generic|traq|codec|parser|extractor|trap-syntax)/,
  'markdown-extractor': /^markdown-(?:commonmark|generic|trap|traq|codec|parser|renderer)/,
  'markdown-renderer': /^markdown-(?:commonmark|generic|trap|traq|codec|parser|extractor)/,
  'markdown-parser': /^markdown-(?:commonmark|generic|trap|traq|codec)/,
  'markdown-commonmark': /^markdown-(?:generic|trap|traq|codec)/,
  'markdown-generic-syntax': /^markdown-(?:trap|traq|codec)/,
  'markdown-trap-syntax': /^markdown-(?:generic|traq|codec)/,
  'markdown-traq': /^markdown-codec$/,
};
for (const [name, forbidden] of Object.entries(boundaries)) {
  const pkg = metadata.packages.find(pkg => pkg.name === name);
  assert.ok(pkg, `Missing package: ${name}`);
  for (const id of dependencies(pkg.id)) {
    const dependency = packages.get(id);
    assert.ok(!forbidden.test(dependency.name), `${name} depends on ${dependency.name}`);
    const location = relative(root, dependency.manifest_path).replaceAll('\\', '/');
    assert.ok(!/^(?:experiments|\.private)\//.test(location), `${name} depends on ${location}`);
  }
  console.log(`${name}: dependency boundary passed`);
}
