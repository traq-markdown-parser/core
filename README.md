# traQ Markdown parser

Rust で文法を定義し、Go と TypeScript / JavaScript から同じ WebAssembly パーサーを使うライブラリです。
CommonMark 0.31.2、汎用の拡張、traQ 固有の拡張を分離し、preset の利用と文法の組み立てに対応します。

- [API と拡張の使い方](docs/implementation.md)
- [実行できる examples](examples/README.md)
- [開発・ビルド・テスト](CONTRIBUTING.md)

## 試す

Node.js 24 以降、Go 1.25 以降、Rust と Bash が必要です。
Rust のバージョンと Wasm target は [rust-toolchain.toml](rust-toolchain.toml) で固定しています。

```sh
npm ci
npm run build
npm run examples
```

Wasm はビルド時に生成します。ソース管理には含めません。

## API

Rust:

```rust
use traq_markdown::presets;

let parser = presets::traq::v1::parser();
let document = parser.parse("**hello** :stamp:")?;
let inline = parser.parse_inline("**hello**")?;
```

TypeScript:

```ts
import { loadRuntime } from '@traptitech/markdown-parser'

const runtime = await loadRuntime(wasmBytes)
try {
  const parser = runtime.parser(runtime.presets.traq.v1)
  try {
    const document = parser.parse('**hello** :stamp:')
  } finally {
    parser.dispose()
  }
} finally {
  runtime.dispose()
}
```

Go:

```go
runtime, err := core.New(ctx, wasmBytes)
if err != nil { return err }
defer runtime.Close(ctx)

parser, err := runtime.Parser(ctx, runtime.Presets.TraQ.V1)
if err != nil { return err }
defer parser.Close()

result, err := parser.Parse(ctx, "**hello** :stamp:")
if err != nil { return err }
```

同じ Parser を繰り返し利用できます。Go の Parser は並行利用にも対応します。
組み込み preset は必要になるまで文法をコンパイルせず、作成済みの構成は共有します。
独自構成の builder.build / Build はその場で検証・構築します。

保存済みメッセージの文法版と Parser の対応は利用側で管理します。
SDK と AST に文法版や永続的な Profile ID はありません。
現在の配布契約は ABI 2 / AST 3。Wasm と生成済み binding を組にして利用します。

## 構成

| ディレクトリ | 内容 |
|---|---|
| crates/markdown | 解析機構、文法、preset、公開テストデータ |
| crates/wasm | Wasm の入出力とリソース管理 |
| go | Go SDK と型付き AST |
| packages/browser | TypeScript / JavaScript SDK |
| packages/renderer | AST から markdown-it token への描画アダプター |
| examples | 各言語の利用例 |
| scripts | ビルド、binding 生成、配布物の検証 |
| tests | JS と配布物のテスト |
| docs | 現行 API の説明 |

このプロジェクトは [MIT License](LICENSE) で公開します。
著作権表記: Copyright (c) 2026 東京工業大学デジタル創作同好会traP。

第三者コードの通知は [THIRD_PARTY_NOTICES.md](crates/markdown/THIRD_PARTY_NOTICES.md)、
公開テストデータの出典・ライセンスは [fixtures/README.md](crates/markdown/tests/fixtures/README.md) にあります。
