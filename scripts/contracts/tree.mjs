import { quoted as q } from './schema.mjs';

// The distribution owns the union. Tree traversal never contains this type list.
export function treeFiles(entries) {
  const header = '// Generated from the Rust node contracts. Do not edit.\n';
  return new Map([
    ['packages/browser/generated/Span.ts', header + 'export type Span = { start: number; end: number };\n'],
    ['packages/browser/generated/NodeKind.ts', header +
      entries.map(([, s]) => `import type { ${s.title} } from './${s.title}.js';`).join('\n') +
      '\nexport type NodeKind =\n' + entries.map(([key, s]) =>
        `  | { kind: ${q(key)}; data: ${s.title} }`).join('\n') + ';\n'],
    ['packages/browser/generated/Node.ts', header +
      "import type { Span } from './Span.js';\nimport type { NodeKind } from './NodeKind.js';\n" +
      'export type Node<AllowUnknown extends boolean = false> =\n' +
      '  (NodeKind | (AllowUnknown extends true ? { kind: string; data: unknown } : never)) &\n' +
      '  { span: Span; children?: Node<AllowUnknown>[] };\n'],
    ['packages/browser/generated/Document.ts', header +
      "import type { Node } from './Node.js';\n" +
      'export type Document<AllowUnknown extends boolean = false> = {\n' +
      '  source: string; children: Node<AllowUnknown>[];\n};\n'],
  ]);
}
